use crate::cli::command::{Command, CommandContext};
use crate::cli::registry::CommandRegistry;
use crate::utils::path_lookup::find_in_path;
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
            if let Some(cmd) = reg.get(cmd_name) {
                if cmd.is_builtin() {
                    writeln!(ctx.stdout, "{} is a shell builtin", cmd_name).ok();
                    return;
                }
            }

            if let Some(path) = find_in_path(cmd_name) {
                writeln!(ctx.stdout, "{} is {}", cmd_name, path.display()).ok();
                return;
            }
        }
        writeln!(ctx.stdout, "{}: command not found", cmd_name).ok();
    }
}
