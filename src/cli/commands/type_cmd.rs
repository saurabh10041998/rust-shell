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
                ctx.stderr.write_line("Usage: type <command-name>").ok();
                return;
            }
        };
        if let Some(reg) = self.registry.upgrade() {
            let reg = reg.borrow();
            if let Some(cmd) = reg.get(cmd_name) {
                if cmd.is_builtin() {
                    ctx.stdout
                        .write_line(format!("{} is a shell builtin", cmd_name).as_str())
                        .ok();
                    return;
                }
            }

            if let Some(path) = find_in_path(cmd_name) {
                ctx.stdout
                    .write_line(format!("{} is {}", cmd_name, path.display()).as_str())
                    .ok();
                return;
            }
        }
        ctx.stderr
            .write_line(format!("{}: not found", cmd_name).as_str())
            .ok();
    }
}
