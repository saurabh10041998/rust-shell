use crate::cli::registry::CommandRegistry;
use std::rc::Rc;

pub mod echo;
pub mod exit;
pub mod type_cmd;

use crate::cli::commands::echo::EchoCommand;
use crate::cli::commands::exit::ExitCommand;

pub fn register_all(registry: &mut CommandRegistry) {
    registry.register(Rc::new(ExitCommand));
    registry.register(Rc::new(EchoCommand));
}
