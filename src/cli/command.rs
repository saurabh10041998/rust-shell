pub struct CommandContext<'a> {
    pub stdin: &'a mut dyn std::io::Read,
    pub stdout: &'a mut dyn std::io::Write,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, args: &[&str], ctx: &mut CommandContext);
    fn is_builtin(&self) -> bool {
        true
    }
}
