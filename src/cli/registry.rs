use crate::cli::command::{Command, CommandContext};
use crate::cli::parser::split_argv::ArgvSplitter;
use crate::utils::path_lookup::find_in_path;
use std::collections::HashMap;
use std::process::Command as ProcCommand;
use std::rc::Rc;

pub struct CommandRegistry {
    commands: HashMap<String, Rc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: Rc<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    pub fn get(&self, name: &str) -> Option<&Rc<dyn Command>> {
        self.commands.get(name)
    }

    pub fn execute(&self, input: &str, ctx: &mut CommandContext) -> bool {
        let argv = match ArgvSplitter::split(input) {
            Ok(v) => v,
            Err(e) => {
                writeln!(ctx.stderr, "parse error: {}", e).ok();
                return true;
            }
        };
        if argv.is_empty() {
            return true;
        }
        let cmd_name = &argv[0];
        let args = argv.iter().skip(1).map(|s| s.as_str()).collect::<Vec<_>>();

        if let Some(cmd) = self.commands.get(cmd_name) {
            cmd.execute(&args, ctx);
            return true;
        }

        if let Some(_) = find_in_path(cmd_name) {
            match ProcCommand::new(cmd_name)
                .args(&args)
                .spawn()
                .and_then(|mut child| child.wait())
            {
                Ok(_status) => {
                    //writeln!(ctx.stdout, "{}", status).ok();
                }
                Err(e) => {
                    writeln!(ctx.stdout, "Failed to run {}: {}", cmd_name, e).ok();
                }
            }
            return true;
        }

        writeln!(ctx.stdout, "{}: not found", cmd_name).ok();
        true
    }
}
