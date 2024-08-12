use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::process;

#[derive(Clone)]
enum Value {
    Int(i32),
    String(String),
}

struct UwuInterpreter {
    variables: HashMap<String, Value>,
}

impl UwuInterpreter {
    fn new() -> Self {
        UwuInterpreter {
            variables: HashMap::new(),
        }
    }

    fn read(&mut self, prompt: &str) -> Value {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        Value::String(input.trim().to_string())
    }

    fn write(&self, value: &Value) {
        match value {
            Value::Int(i) => println!("{}", i),
            Value::String(s) => println!("{}", s),
        }
    }

    fn to_int(&self, value: &Value) -> i32 {
        match value {
            Value::Int(i) => *i,
            Value::String(s) => s.parse().unwrap_or(0),
        }
    }

    fn evaluate(&self, expr: &str) -> Value {
        if expr.starts_with('"') && expr.ends_with('"') {
            Value::String(expr[1..expr.len()-1].to_string())
        } else if let Ok(i) = expr.parse::<i32>() {
            Value::Int(i)
        } else if let Some(value) = self.variables.get(expr) {
            value.clone()
        } else {
            Value::Int(0)
        }
    }

    fn execute(&mut self, code: &str) {
        for line in code.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 1 {
                // Function call
                let mut parts = parts[0].split('(');
                let func = parts.next().unwrap().trim();
                let args = parts.next().unwrap_or("").trim_end_matches(')');
                match func {
                    "write" => self.write(&self.evaluate(args)),
                    "read" => {
                        let value = self.read("> ");
                        self.variables.insert(args.to_string(), value);
                    },
                    _ => (),
                }
            } else {
                // Variable assignment
                let var = parts[0].trim();
                let expr = parts[1].trim();
                let value = self.evaluate(expr);
                self.variables.insert(var.to_string(), value);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.uwu>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let program = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    let mut interpreter = UwuInterpreter::new();
    interpreter.execute(&program);
}