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
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let path = &args[1];
        println!("Running file: {}", path);
        match read_file(path) {
            Ok(contents) => println!("File contents:\n{}", contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else {
        println!("No path argument provided.");
    }
}
