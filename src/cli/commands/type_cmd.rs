use crate::cli::command::Command;
use crate::cli::registry::CommandRegistry;
use std::cell::RefCell;
use std::rc::Weak;

pub struct TypeCommand {
    pub registry: Weak<RefCell<CommandRegistry>>,
}

impl Command for TypeCommand {
    fn name(&self) -> &'static str {
        "type"
    }

    fn description(&self) -> &'static str {
        "Show if command is built-in"
    }

    fn execute(&self, args: &[&str]) {
        let cmd_name = match args.get(0) {
            Some(cmd_str) => cmd_str,
            None => {
                println!("Usage: type <command-name>");
                return;
            }
        };
        if let Some(reg) = self.registry.upgrade() {
            let reg = reg.borrow();
            match reg.get(cmd_name) {
                Some(cmd) if cmd.is_builtin() => {
                    println!("{} is a shell builtin", cmd_name);
                }
                Some(_) => {
                    unreachable!();
                }
                None => {
                    println!("{}: not found", cmd_name)
                }
            }
        }
    }
}
