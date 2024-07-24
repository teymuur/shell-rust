use std::io::{self, stdin, stdout};
use std::process::{Command};

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // read_line leaves a trailing newline, which trim removes
    let command = input.trim(); 

    Command::new(command)
        .spawn()
        .unwrap();
}