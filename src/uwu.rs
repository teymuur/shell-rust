use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_file(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
use std::collections::HashMap;

fn execute(script: &str) {
    let mut variables: HashMap<String, i32> = HashMap::new();

    for line in script.lines() {
        let mut parts = line.split_whitespace();
        if let Some(command) = parts.next() {
            match command {
                "write" => {
                    if let Some(value) = parts.next() {
                        print!("{}", value);
                    }
                },
                "set" => {
                    if let (Some(var_name), Some(value_str)) = (parts.next(), parts.next()) {
                        if let Ok(value) = value_str.parse::<i32>() {
                            variables.insert(var_name.to_string(), value);
                        } else {
                            eprintln!("Invalid value for 'set' command: {}", value_str);
                        }
                    }
                },
                "add" => {
                    if let (Some(var_name), Some(value_str)) = (parts.next(), parts.next()) {
                        if let Ok(value) = value_str.parse::<i32>() {
                            *variables.entry(var_name.to_string()).or_insert(0) += value;
                        } else {
                            eprintln!("Invalid value for 'add' command: {}", value_str);
                        }
                    }
                },
                "show" => {
                    if let Some(var_name) = parts.next() {
                        match variables.get(var_name) {
                            Some(value) => println!("{} = {}", var_name, value),
                            None => println!("{} is not defined", var_name),
                        }
                    }
                },
                _ => eprintln!("Unknown command: {}", command),
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let path = &args[1];
        println!("Running file: {}", path);
        match read_file(path) {
            Ok(contents) => execute(&contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else {
        println!("No path argument provided.");
    }
}
