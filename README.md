# Rust Shell

This Rust program provides a simple shell-like interface with basic command support. It supports changing directories, listing directory contents, creating new directories, executing commands as an administrator, clearing the screen, writing messages, and exiting the shell.

## Installation

To run this program, you need to have Rust installed on your system. You can download it from the official Rust website: https://www.rust-lang.org/tools/install

Once Rust is installed, you can clone this repository or download the source code. Then, navigate to the directory containing the source code and run the following command to build the program:

```sh
$ cargo build --release
```

This will create a `main` executable in the `target/release` directory. You can run the program by executing:

```sh
$ ./target/release/main
```

## Usage

When you run the program, you will see a prompt similar to this:

```
TS:::C:\Users\YourUsername\Documents\rust-shell\target\release\-->
```

This indicates that the current directory is `C:\Users\YourUsername\Documents\rust-shell\target\release`. You can enter commands to interact with the shell.

Here are the available commands:

- `cd <directory>`: Change the current directory to the specified directory.
- `ls [<directory>]`: List the contents of the specified directory (or the current directory if no directory is provided).
- `nwdir <directory_name>`: Create a new directory with the specified name.
- `imgod`: Execute the `main.exe` program as an administrator.
- `white`: Clear the screen.
- `write <message>`: Write the specified message to the console.
- `uwu <path>`: Run uwu script from an uwu file.
- `mkfile <filename>`  - Create a new file
- `edfile <filename>`  - Edit an existing file or create a new one
- `help`: Get a list of commands
- `exit`: Exit the shell.

You can also chain commands together using the `^_^` separator. For example:

```
ls ^_^ cd .. ^_^ ls
```

This will list the contents of the current directory, change to the parent directory, and then list the contents of the parent directory.

## uwu code
Commands

- write <string>: Outputs the specified string to the console.
- set <variable> <value>: Sets the variable to the specified integer value.
- add <variable> <value>: Adds the specified integer value to the variable.
- show <variable>: Displays the current value of the variable