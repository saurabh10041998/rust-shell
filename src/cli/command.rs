use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub enum IoHandle {
    Stdout,
    Stderr,
    Stdin,
    File(Rc<RefCell<File>>),
    Null,
}

impl IoHandle {
    pub fn write_line(&mut self, msg: &str) -> io::Result<()> {
        match self {
            IoHandle::Stdout => writeln!(io::stdout(), "{}", msg),
            IoHandle::Stderr => writeln!(io::stderr(), "{}", msg),
            IoHandle::File(file) => {
                let mut _file = file.borrow_mut();
                writeln!(_file, "{}", msg)
            }
            IoHandle::Null => Ok(()),
            _ => unreachable!("Cannot write to stdin stream"),
        }
    }
}

pub struct CommandContext {
    pub stdin: IoHandle,
    pub stdout: IoHandle,
    pub stderr: IoHandle,
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
