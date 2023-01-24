# cee project manager

cee is a highly customizable project manager for C and C++ projects with extremely similar usage to [cargo](https://github.com/rust-lang/cargo/)

## Installation

Unix-like

```bash
$ git clone https://github.com/basicallygit/cee-project-manager
$ cd cee-project-manager
$ chmod +x install.sh
$ ./install.sh
```

Windows

```powershell
$ git clone https://github.com/basicallygit/cee-project-manager
$ cd cee-project-manager
$ powershell.exe -ExecutionPolicy Bypass -File "install.ps1"
```

Manual

```bash
$ git clone https://github.com/basicallygit/cee-project-manager
$ cd cee-project-manager
$ cargo build --release
```


## Usage

### commands

|Command|Description|
|---|---|
|cee <kbd>new (project name)</kbd>|Creates a new project|
|cee <kbd>init</kbd>|Initializes project in the pwd|
|cee <kbd>build</kbd>|Build the project but dont run it|
|cee <kbd>run</kbd><kbd> (args) </kbd>|Build and run the project (args will be passed to the program)|
|cee <kbd>clean</kbd>|Purge the bin folder of all compiled files|

### flags

|Flag|Description|
|---|---|
|<kbd>--release/-r</kbd>|Compile with optimizations turned on (run/build only)|
|<kbd>--lang/-l</kbd>|Specify language (default=c) (init/new only)|
|<kbd>--compiler-output/-co</kbd>|Show exit code, stdout & stderr output of the compiler|

## cee.conf customization

|Key|Description|
|---|---|
|<kbd>C_COMPILER</kbd>|The C compiler to use|
|<kbd>CPP_COMPILER</kbd>|The C++ compiler to use|
|<kbd>RELEASE_FLAGS</kbd>|Flags for the compiler when in release mode|
|<kbd>DEBUG_OUTPUT_FILE</kbd>|Output binary for non-release mode (debug)|
|<kbd>RELEASE_OUTPUT_FILE</kbd>|Output binary for release mode|
|<kbd>INPUT_SRC_FILE</kbd>|The main source file|
|<kbd>VERSION</kbd>|Project version|

### tested on

|OS|extra info|
|---|---|
|Linux|no problems|
|Windows|no problems|
|FreeBSD|no problems|
|OpenBSD|no problems|
|Android|no problems|

ios, macos, dragonfly/netBSD and other posix-compliant OS's will also more than likely work fine
