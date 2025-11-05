use crate::cli::command::Command;
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    pub fn execute(&self, input: &str) -> bool {
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
