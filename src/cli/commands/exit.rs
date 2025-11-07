use crate::cli::command::{Command, CommandContext};

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "exits shell with given exit status"
    }

    fn execute(&self, args: &[&str], _ctx: &mut CommandContext) {
        let status_code = match args.get(0) {
            Some(status_code_str) => status_code_str.parse::<i32>().expect("Invalid status code"),
            None => {
                // TODO: record last cmd status code
                // and exit with the same instead of 0
                0
            }
        };
        std::process::exit(status_code);
    }
}
