use crate::cli::registry::CommandRegistry;

pub mod exit;

use crate::cli::commands::exit::ExitCommand;

pub fn register_all(registry: &mut CommandRegistry) {
    registry.register(Box::new(ExitCommand));
}
