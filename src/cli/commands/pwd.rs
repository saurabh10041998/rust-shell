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
            ctx.stdout.write_line(format!("{}", pwd).as_str()).ok();
            return;
        }
        match env::current_dir() {
            Ok(path) => {
                ctx.stdout
                    .write_line(format!("{}", path.display()).as_str())
                    .ok();
            }
            Err(e) => {
                ctx.stdout.write_line(format!("pwd: {}", e).as_str()).ok();
            }
        }
    }
}
