use std::io::{self, Write, Read, Seek, SeekFrom,stdin,stdout};
use std::process::{Command, Stdio, Child};
use std::env;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};


fn create_file(filename: &str) -> io::Result<()> {
    File::create(filename)?;
    println!("File '{}' created successfully.", filename);
    Ok(())
}

fn edit_file(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    println!("Current contents of '{}':", filename);
    println!("{}", contents);
    println!("Enter new contents (type 'EOF' on a new line to finish):");
    
    let mut new_contents = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim() == "EOF" {
            break;
        }
        new_contents.push_str(&line);
    }
    
    file.set_len(0)?;
    file.seek(io::SeekFrom::Start(0))?;
    file.write_all(new_contents.as_bytes())?;
    println!("File '{}' updated successfully.", filename);
    Ok(())
}

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
        let mut commands = input.trim().split("||").peekable();
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
                    let output = Command::new("sudo")
                    .arg("su")
                    .spawn();
            
                match output {
                    Ok(_) => print!("SUDO:"),
                    Err(e) => eprintln!("Failed to run main as superuser: {}", e),
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
                "mkfile" => {
                    if let Some(filename) = args.get(0) {
                        match create_file(filename) {
                            Ok(_) => (),
                            Err(e) => eprintln!("Error creating file: {}", e),
                        }
                    } else {
                        eprintln!("Usage: mkfile <filename>");
                    }
                    previous_command = None;
                },
                "edfile" => {
                    if let Some(filename) = args.get(0) {
                        match edit_file(filename) {
                            Ok(_) => (),
                            Err(e) => eprintln!("Error editing file: {}", e),
                        }
                    } else {
                        eprintln!("Usage: edfile <filename>");
                    }
                    previous_command = None;
                }
                "help" => {
                    println!("Available commands:");
                    println!("  cd <directory>     - Change the current directory");
                    println!("  ls [directory]     - List contents of the current or specified directory");
                    println!("  nwdir <directory>  - Create a new directory");
                    println!("  imgod              - Run main.exe as administrator");
                    println!("  white              - Clear the screen");
                    println!("  write <message>    - Print a message to the console");
                    println!("  uwu <path>         - Run code from uwu file");
                    println!("  mkfile <filename>  - Create a new file");
                    println!("  edfile <filename>  - Edit an existing file or create a new one");
                    println!("  help               - Display this help message");
                    println!("  exit               - Exit the shell");
                    println!("Any other command available on your default terminal is usable");
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
