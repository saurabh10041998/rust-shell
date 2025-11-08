use crate::cli::registry::CommandRegistry;
use std::rc::Rc;

pub mod echo;
pub mod exit;
pub mod pwd;
pub mod type_cmd;

use crate::cli::commands::echo::EchoCommand;
use crate::cli::commands::exit::ExitCommand;
use crate::cli::commands::pwd::PwdCommand;

pub fn register_all(registry: &mut CommandRegistry) {
    registry.register(Rc::new(ExitCommand));
    registry.register(Rc::new(EchoCommand));
    registry.register(Rc::new(PwdCommand));
}
