use std::collections::HashMap;

pub struct CommandContext<'a> {
    pub stdin: &'a mut dyn std::io::Read,
    pub stdout: &'a mut dyn std::io::Write,
    pub stderr: &'a mut dyn std::io::Write,
    pub env: HashMap<String, String>,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, args: &[&str], ctx: &mut CommandContext);
    fn is_builtin(&self) -> bool {
        true
    }
}
