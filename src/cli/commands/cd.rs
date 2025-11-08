use crate::cli::command::{Command, CommandContext};
use std::env;

pub struct CdCommand;

impl Command for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn description(&self) -> &'static str {
        "Change the shell working directory"
    }

    fn execute(&self, args: &[&str], ctx: &mut CommandContext) {
        let target = if args.is_empty() {
            ctx.env.get("HOME").map(|s| s.as_str()).unwrap_or("/")
        } else {
            args[0]
        };

        if let Err(e) = env::set_current_dir(target) {
            writeln!(ctx.stderr, "cd: {}: {}", target, e).ok();
            return;
        }

        // Update ctx's pwd
        if let Ok(new_dir) = env::current_dir() {
            ctx.env.insert("PWD".into(), new_dir.display().to_string());
        }
    }
}
