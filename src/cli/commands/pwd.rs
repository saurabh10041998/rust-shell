use crate::cli::command::{Command, CommandContext};
use std::env;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn description(&self) -> &'static str {
        "Print the current working directory"
    }

    fn execute(&self, _args: &[&str], ctx: &mut CommandContext) {
        if let Some(pwd) = ctx.env.get("PWD") {
            writeln!(ctx.stdout, "{}", pwd).ok();
            return;
        }
        match env::current_dir() {
            Ok(path) => {
                writeln!(ctx.stdout, "{}", path.display()).ok();
            }
            Err(e) => {
                writeln!(ctx.stdout, "pwd: {}", e).ok();
            }
        }
    }
}
