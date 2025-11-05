use std::io::{self, Write};
mod cli;
use cli::commands;
use cli::registry::CommandRegistry;

fn main() {
    let mut registry = CommandRegistry::new();
    commands::register_all(&mut registry);
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
