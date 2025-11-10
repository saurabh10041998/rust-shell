use std::collections::HashMap;

pub struct CommandContext {
    pub stdin: Box<dyn std::io::Read>,
    pub stdout: Box<dyn std::io::Write>,
    pub stderr: Box<dyn std::io::Write>,
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
