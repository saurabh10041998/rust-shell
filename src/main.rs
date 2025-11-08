use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
mod cli;
mod utils;
use cli::command::CommandContext;
use cli::commands;
use cli::commands::type_cmd::TypeCommand;
use cli::registry::CommandRegistry;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut ctx = CommandContext {
        stdin: &mut stdin,
        stdout: &mut stdout,
    };

    let mut registry = CommandRegistry::new();
    commands::register_all(&mut registry);
    // Register 'type' command seperately as it holds
    // back reference to CommandRegistry
    let reg_rc = Rc::new(RefCell::new(registry));
    let type_cmd = Rc::new(TypeCommand {
        registry: Rc::downgrade(&reg_rc),
    });
    reg_rc.borrow_mut().register(type_cmd);

    let _reg_rc = reg_rc.borrow();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        if !_reg_rc.execute(&line, &mut ctx) {
            break;
        }
    }
}
