use crate::cli::command::{Command, CommandContext};
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

    fn execute(&self, args: &[&str], ctx: &mut CommandContext) {
        let cmd_name = match args.get(0) {
            Some(cmd_str) => cmd_str,
            None => {
                writeln!(ctx.stdout, "Usage: type <command-name>").ok();
                return;
            }
        };
        if let Some(reg) = self.registry.upgrade() {
            let reg = reg.borrow();
            match reg.get(cmd_name) {
                Some(cmd) if cmd.is_builtin() => {
                    writeln!(ctx.stdout, "{} is a shell builtin", cmd_name).ok();
                }
                Some(_) => {
                    unreachable!();
                }
                None => {
                    writeln!(ctx.stdout, "{}: not found", cmd_name).ok();
                }
            }
        }
    }
}
