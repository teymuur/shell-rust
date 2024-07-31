use std::io::{Write, stdin, stdout};
use std::process::{Command, Stdio, Child};
use std::env;
use std::path::Path;
use std::fs;



fn list_dir(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                print!("{}/ ", entry.file_name().to_string_lossy());
                            } else {
                                print!("{} ", entry.file_name().to_string_lossy());
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
            println!();  // Newline after listing directory contents
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}

fn main() {
    println!("------------------------------------------------\n");
    loop {
        // Get the current directory
        let current_dir = env::current_dir().unwrap();
        print!("TS::{}\\--> ", current_dir.display());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" ^_^ ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap_or("cd");
            let args: Vec<&str> = parts.collect();

            match command {
                "cd" => {
                    let new_dir = args.get(0).map_or(".", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
           
                "ls" => {
                    let path = args.get(0).unwrap_or(&".");
                    list_dir(path);
                    previous_command = None;
                },
                "nwdir" => {
                    if let Some(new_dir) = args.get(0) {
                        match fs::create_dir(new_dir) {
                            Ok(_) => println!("Directory '{}' created.", new_dir),
                            Err(e) => eprintln!("Failed to create directory '{}': {}", new_dir, e),
                        }
                    } else {
                        eprintln!("Usage: nwdir <directory_name>");
                    }
                    previous_command = None;
                },
                "imgod" => {
                    let output = Command::new("runas")
                        .arg("/user:Administrator")
                        .arg("./main.exe")
                        .output();

                    match output {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            if !stdout.is_empty() {
                                print!("{}", stdout);
                            }
                            if !stderr.is_empty() {
                                eprintln!("{}", stderr);
                            }
                        },
                        Err(e) => eprintln!("Failed to execute 'main.exe' as administrator: {}", e),
                    }
                    previous_command = None;
                },
                "white" => {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    previous_command = None;
                },
                "write" => {
                    let message = args.join(" ");
                    println!("{}", message);
                    previous_command = None;
                },
                "helpme" => {
                    println!("Available commands:");
                    println!("  cd <directory>     - Change the current directory");
                    println!("  ls [directory]     - List contents of the current or specified directory");
                    println!("  nwdir <directory>  - Create a new directory");
                    println!("  imgod              - Run main.exe as administrator");
                    println!("  white              - Clear the screen");
                    println!("  write <message>    - Print a message to the console");
                    println!("  help               - Display this help message");
                    println!("  exit               - Exit the shell");
                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(&args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("Failed to execute '{}': {}", command, e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}
