use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(non_snake_case)]
pub struct Config {
    pub LANG: String,
    pub C_COMPILER: String,
    pub CPP_COMPILER: String,
    pub RELEASE_FLAGS: String,
    pub DEBUG_OUT_FILE: String,
    pub RELEASE_OUT_FILE: String,
    pub INPUT_SRC_FILE: String,
    pub VERSION: String,
}

pub fn get_value(file_line: &String) -> String {
    if let Some(index) = file_line.find('=') {
        let x = &file_line[index+1..file_line.len()];
        return x.trim().to_string();
    }
    String::new()
}

#[allow(non_snake_case)]
pub fn get_config(filename: &str) -> Config {
    let mut LANGUAGE = String::new();
    let mut CC = String::new();
    let mut CPPC = String::new();
    let mut RFLGS = String::new();
    let mut DBOF = String::new();
    let mut RLOF = String::new();
    let mut INPF = String::new();
    let mut VERS = String::new();

    let lines = BufReader::new(File::open(filename).expect("file not found")).lines();

    //go through each line and extract the value
    for line in lines {
        if let Ok(linecontents) = line {

            if linecontents.starts_with("LANG") {
                LANGUAGE = get_value(&linecontents);
            } else if linecontents.starts_with("C_COMPILER") {
                CC = get_value(&linecontents);
            } else if linecontents.starts_with("CPP_COMPILER") {
                CPPC = get_value(&linecontents);
            } else if linecontents.starts_with("RELEASE_FLAGS") {
                RFLGS = get_value(&linecontents);
            } else if linecontents.starts_with("DEBUG_OUTPUT_FILE") {
                DBOF = get_value(&linecontents);
            } else if linecontents.starts_with("RELEASE_OUTPUT_FILE") {
                RLOF = get_value(&linecontents);
            } else if linecontents.starts_with("INPUT_SOURCE_FILE") {
                INPF = get_value(&linecontents);
            } else if linecontents.starts_with("VERS") {
                VERS = get_value(&linecontents);
            }
        }
    }


    //return config
    Config {
        LANG: LANGUAGE,
        C_COMPILER: CC,
        CPP_COMPILER: CPPC,
        RELEASE_FLAGS: RFLGS,
        DEBUG_OUT_FILE: DBOF,
        RELEASE_OUT_FILE: RLOF,
        INPUT_SRC_FILE: INPF,
        VERSION: VERS,
    }

}