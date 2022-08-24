# cee project manager

cee is a highly customizable project manager for C and C++ projects with extremely similar usage to cargo

## Installation

Unix-like

```bash
$ git clone https://github.com/basicallygit/cee-project-manager
$ cd cee-project-manager
$ chmod +x setup.sh
$ ./setup.sh
```

Windows

```bash
$ git clone https://github.com/basicallygit/cee-project-manager
$ cd cee-project-manager
$ cargo build --release
```
Add target/release/cee.exe to your PATH environment variable on windows


## Usage

### commands

|Command|Description|
|---|---|
|cee <kbd>new (project name)</kbd>|Creates a new project|
|cee <kbd>init</kbd>|Initializes project in the pwd|
|cee <kbd>build</kbd>|Build the project but dont run it|
|cee <kbd>run</kbd>|Build and run the project|
|cee <kbd>clean</kbd>|Purge the bin folder of all compiled files|

### flags

|Flag|Description|
|---|---|
|<kbd>--release/-r</kbd>|Compile with optimizations turned on (run/build only)|
|<kbd>--lang/-l</kbd>|Specify language (default=c) (init/new only)|
