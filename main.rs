use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        // Print the prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim the input and split into command and arguments
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => {
                println!("Exiting...");
                break;
            }
            command => {
                // Execute the command
                let mut child = Command::new(command)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn();

                match child {
                    Ok(mut handle) => {
                        handle.wait().unwrap();
                    }
                    Err(e) => {
                        println!("Error executing command: {}", e);
                    }
                }
            }
        }
    }
}
