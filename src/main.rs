extern crate console;
mod configparser;

use std::env::{args, consts, current_dir};
use std::fs::{create_dir, metadata, File, remove_dir_all};
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::time::Instant;
use console::style;

fn main() {
    let info = style("[info]").green();
    let error = style("[error]").red();

    const DEFAULT_CONFIG: &str = "C_COMPILER = gcc
CPP_COMPILER = g++
RELEASE_FLAGS = -O2
#main.exe or main.out etc will be determined at compile time
DEBUG_OUTPUT_FILE = bin/debug/main
RELEASE_OUTPUT_FILE = bin/release/main
INPUT_SOURCE_FILE = src/main
VERSION = 1.0.0";

    let argv: Vec<String> = args().collect();
    let argc = argv.len();

    if argc < 2 || argv[1] == "help" || argv[1] == "--help" || argv[1] == "-h" {
        println!("\n{} {} <command> [flags]", style("Usage:").green(), argv[0]);
        println!("\n{}", style("commands:").green());
        println!("    init: initialize project in the current dir");
        println!("    new <projectname>: create new project");
        println!("    run: compile and run current project");
        println!("    build: compile but do not run");
        println!("    clean: purge the bin folder");
        println!("{}", style("flags:").green());
        println!("    --lang/-l: specify project lang (init/new only)");
        println!("    --release/-r: compile with optimizations (run/build only)");
        println!("    --compiler-output/-co: show the output of the compilation phase\n");
    }
    else if argv[1] == "init" || argv[1] == "new" {
        //default lang is C, change to "cpp" for C++ to be default
        let mut lang = "c";

        if argv[1] == "init" {
            if argc >= 4 {
                if argv[2] == "-l" || argv[2] == "--lang" {
                    let langflag = &argv[3].to_lowercase();
                    if  langflag == "cpp" {
                        lang = "cpp";
                    } else if langflag == "c" {} //ignore, default lang is already c
                    else {
                        eprintln!("{} unknown/unsupported language: '{}'", error, langflag);
                        exit(0);
                    }
                }
            }
            if metadata("cee.conf").is_ok() {
                eprintln!("{} project already found in current directory (cee.conf exists)", error);
                exit(0);
            }
            else {
                create_dir("src").expect("unable to create src directory");
                println!("{} mkdir src", info);
                create_dir("bin").unwrap();
                println!("{} mkdir bin", info);
                create_dir("bin/release").unwrap();
                println!("{} mkdir bin/release", info);
                create_dir("bin/debug").unwrap();
                println!("{} mkdir bin/debug", info);
                let mut src_file = File::create(format!("src/main.{}", lang)).unwrap();
                let src_contents = if lang == "c" { "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}" } else { "#include <iostream>\n\nusing namespace std;\n\nint main() {\n    cout << \"Hello, World!\" << endl;\n    return 0;\n}" };
                write!(src_file, "{}", src_contents).unwrap();
                println!("{} touch src/main.{}", info, lang);
                let mut conf_file = File::create("cee.conf").unwrap();
                let conf_contents = format!("LANG = {}\n{}", lang, DEFAULT_CONFIG);
                write!(conf_file, "{}", conf_contents).unwrap();
                println!("{} touch cee.conf", info);
            }
        }
        //"new"
        else {
            if argc < 3 {
                eprintln!("{} {} new <project name>", style("Usage:").green(), argv[0]);
                exit(0);
            }
            if argc >= 5 {
                if argv[3] == "-l" || argv[3] == "--lang" {
                    let langflag = argv[4].to_lowercase();
                    if langflag == "cpp" || langflag == "c" {
                        lang = "cpp";
                    }
                    else {
                        eprintln!("{} unknown/unsupported language: '{}'", error, langflag);
                        exit(0);
                    }
                }
            }
            if metadata(&argv[2]).is_ok() {
                eprintln!("{} directory already exists: '{}'", error, argv[2]);
                exit(0);
            }
            else {
                let project_name = &argv[2];
                create_dir(project_name).expect("unable to create project folder");
                println!("{} mkdir {}", info, project_name);
                create_dir(format!("{}/src", project_name)).unwrap();
                println!("{} mkdir {}/src", info, project_name);
                create_dir(format!("{}/bin", project_name)).unwrap();
                println!("{} mkdir {}/bin", info, project_name);
                create_dir(format!("{}/bin/release", project_name)).unwrap();
                println!("{} mkdir {}/bin/release", info, project_name);
                create_dir(format!("{}/bin/debug", project_name)).unwrap();
                println!("{} mkdir {}/bin/debug", info, project_name);
                let mut src_file = File::create(format!("{}/src/main.{}", project_name, lang)).unwrap();
                let src_contents = if lang == "c" { "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}" } else { "#include <iostream>\n\nusing namespace std;\n\nint main() {\n    cout << \"Hello, World!\" << endl;\n    return 0;\n}" };
                write!(src_file, "{}", src_contents).unwrap();
                println!("{} touch {}/src/main.{}", info, project_name, lang);
                let mut conf_file = File::create(format!("{}/cee.conf", project_name)).unwrap();
                let conf_contents = format!("LANG = {}\n{}", lang, DEFAULT_CONFIG);
                write!(conf_file, "{}", conf_contents).unwrap();
                println!("{} touch {}/cee.conf", info, project_name);
            }
        }
    }
    else if argv[1] == "run" || argv[1] == "build" {
        if !metadata("cee.conf").is_ok() {
            eprintln!("{} cee.conf not found, are you in the root of your project?", error);
            exit(0);
        }
        let now = Instant::now();
        //remake binary folders if cee clean was invoked
        if !metadata("bin").is_ok() {
            create_dir("bin").unwrap();
        }
        if !metadata("bin/debug").is_ok() {
            create_dir("bin/debug").unwrap();
        }
        if !metadata("bin/release").is_ok() {
            create_dir("bin/release").unwrap();
        }
        let binary_extension = if consts::OS == "windows" { "exe" } else { "out" };
        let mut release = false;
        let config = configparser::get_config("cee.conf");
        let mut cmd_line_args = String::new();
        let exec = style("[exec]").green();
        if argc >= 3 {
            if argv[2] == "--release" || argv[2] == "-r" {
                release = true;
                if argc >= 4 {
                    for i in 3..argc {
                        cmd_line_args.push_str(&argv[i]);
                        cmd_line_args.push_str(" ");
                    }
                }
            }
            else {
                for i in 2..argc {
                    cmd_line_args.push_str(&argv[i]);
                    cmd_line_args.push_str(" ");
                }
            }
        }
        cmd_line_args = cmd_line_args.trim().to_string();
        let compiler = if config.LANG == "c" { config.C_COMPILER } else { config.CPP_COMPILER };
        println!("{} {} v{}", style("Compiling").green(), current_dir().unwrap().display(), config.VERSION);
        if release {
            let mut releaseflags: Vec<&str> = Vec::new();
            for i in config.RELEASE_FLAGS.split(" ") {
                releaseflags.push(i);
            }
            println!("{} {} {}.{} -o {}.{} {}", exec, compiler, config.INPUT_SRC_FILE, config.LANG, config.RELEASE_OUT_FILE, binary_extension, config.RELEASE_FLAGS);
            let output = Command::new(&compiler)
                .arg(format!("{}.{}", config.INPUT_SRC_FILE, config.LANG))
                .arg("-o")
                .arg(format!("{}.{}", config.RELEASE_OUT_FILE, binary_extension))
                .args(releaseflags)
                .stdout(Stdio::piped())
                .output().ok().expect(format!("{} {}", error, style("building failed, do you have your compiler in your PATH variable?").color256(208)).as_str());
            // debug compiler output
            if argv.contains(&"-co".to_string()) || argv.contains(&"--compiler-output".to_string()) {
                println!("Compiler {}", output.status); //output.status already contains the string "exit code: n"
                println!("stdout: {}", style(String::from_utf8(output.stdout).unwrap()).cyan());
                println!("stderr: {}", style(String::from_utf8(output.stderr.clone()).unwrap()).color256(208));
                if !output.stderr.is_empty() { exit(0); }
            }
            if !output.stderr.is_empty() {
                eprintln!("{} compilation error. check -co/--compiler-output to see errors", error);
                exit(0);
            }
        }
        else {
            println!("{} {} {}.{} -o {}.{}", exec, compiler, config.INPUT_SRC_FILE, config.LANG, config.DEBUG_OUT_FILE, binary_extension);
            let output = Command::new(&compiler)
                .arg(format!("{}.{}", config.INPUT_SRC_FILE, config.LANG))
                .arg("-o")
                .arg(format!("{}.{}", config.DEBUG_OUT_FILE, binary_extension))
                .output().ok().expect(format!("{} {}", error, style("building failed, do you have your compiler in your PATH variable?").color256(208)).as_str());
            // debug compiler output
            if argv.contains(&"-co".to_string()) || argv.contains(&"--compiler-output".to_string()) {
                println!("Compiler {}", output.status); //output.status already contains the string "exit code: n"
                println!("stdout: {}", style(String::from_utf8(output.stdout).unwrap()).cyan());
                println!("stderr: {}", style(String::from_utf8(output.stderr.clone()).unwrap()).color256(208));
                if !output.stderr.is_empty() { exit(0); }
            }
            if !output.stderr.is_empty() {
                eprintln!("{} compilation error. check -co/--compiler-output to see errors", error);
                exit(0);
            }
        }
        let comptype = if release { "release" } else { "debug" };
        println!("{} {} [{}] in {}s", style("Finished").green(), current_dir().unwrap().display(), comptype, now.elapsed().as_millis() as f32 / 1000_f32);

        if argv[1] == "run" {
            let mut outputbinary = if release { config.RELEASE_OUT_FILE } else { config.DEBUG_OUT_FILE };
            if consts::OS == "windows" {
                outputbinary = outputbinary.replace("/", "\\");
            }
            println!("{} {}.{} {}", exec, outputbinary, binary_extension, cmd_line_args);
            if cmd_line_args != "" {
                let mut cmdlineargs: Vec<&str> = Vec::new();
                for i in cmd_line_args.split(" ") {
                    cmdlineargs.push(i);
                }
                Command::new(format!("{}.{}", outputbinary, binary_extension))
                    .args(cmdlineargs)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
            else {
                Command::new(format!("{}.{}", outputbinary, binary_extension))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
    }
    else if argv[1] == "clean" {
        if metadata("cee.conf").is_ok() {
            println!("{} purging ./bin ...", info);
            remove_dir_all("bin").unwrap();
            println!("{} Finished", info);
        }
        else {
            eprintln!("{} cee.conf not found, are you in the root of your project?", error);
        }
    }
    else {
        eprintln!("{} Unknow command: '{}'", error, &argv[1]);
    }
}
