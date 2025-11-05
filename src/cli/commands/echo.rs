use crate::cli::command::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Echos input arguments"
    }

    fn execute(&self, args: &[&str]) {
        println!("{}", args.join(" "));
    }
}
