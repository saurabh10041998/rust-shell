use crate::cli::command::{Command, CommandContext};
use crate::utils::path_lookup::expand_tilde;
use std::env;
use std::io::ErrorKind;

pub struct CdCommand;

impl Command for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn description(&self) -> &'static str {
        "Change the shell working directory"
    }

    fn execute(&self, args: &[&str], ctx: &mut CommandContext) {
        let raw_target = if args.is_empty() {
            ctx.env.get("HOME").map(|s| s.as_str()).unwrap_or("/")
        } else {
            args[0]
        };

        let target = expand_tilde(raw_target);

        if let Err(e) = env::set_current_dir(&target) {
            match e.kind() {
                ErrorKind::NotFound => ctx
                    .stderr
                    .write_line(format!("cd: {}: No such file or directory", target).as_str())
                    .ok(),
                _ => ctx
                    .stderr
                    .write_line(format!("cd: {}: {}", target, e).as_str())
                    .ok(),
            };
            return;
        }

        // Update ctx's pwd
        if let Ok(new_dir) = env::current_dir() {
            ctx.env.insert("PWD".into(), new_dir.display().to_string());
        }
    }
}
