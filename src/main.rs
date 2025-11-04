use std::collections::HashMap;
use std::io::{self, Write};

trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str]);
}

struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    fn execute(&self, input: &str) -> bool {
        let mut parts = input.trim().split_whitespace();
        let cmd_name = match parts.next() {
            Some(c) => c,
            None => return true,
        };
        let args: Vec<&str> = parts.collect();

        if let Some(cmd) = self.commands.get(cmd_name) {
            cmd.execute(&args);
        } else {
            println!("{}: command not found", cmd_name);
        }
        true
    }
}

fn main() {
    let registry = CommandRegistry::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        if !registry.execute(&line) {
            break;
        }
    }
}
