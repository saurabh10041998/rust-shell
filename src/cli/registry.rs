use crate::cli::command::{Command, CommandContext};
use crate::cli::parser::parse_simple::{parse_command, ParsedCommand, RedirKind};
use crate::cli::parser::tokenize::ArgvTokenizer;
use crate::utils::path_lookup::find_in_path;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command as ProcCommand;
use std::rc::Rc;

pub struct CommandRegistry {
    commands: HashMap<String, Rc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: Rc<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    pub fn get(&self, name: &str) -> Option<&Rc<dyn Command>> {
        self.commands.get(name)
    }

    pub fn execute_parsed(&self, parsed: ParsedCommand, ctx: &mut CommandContext) -> bool {
        let mut old_stdin: Option<Box<dyn Read>> = None;
        let mut old_stdout: Option<Box<dyn Write>> = None;
        let mut old_stderr: Option<Box<dyn Write>> = None;

        for redir in &parsed.redirects {
            match redir.kind {
                RedirKind::StdoutTruncate => {
                    let file = File::create(&redir.target).unwrap();
                    old_stdout = Some(std::mem::replace(&mut ctx.stdout, Box::new(file)));
                }
                RedirKind::StdoutAppend => {
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&redir.target)
                        .unwrap();
                    old_stdout = Some(std::mem::replace(&mut ctx.stdout, Box::new(file)));
                }
                RedirKind::StderrTruncate => {
                    let file = File::create(&redir.target).unwrap();
                    old_stderr = Some(std::mem::replace(&mut ctx.stderr, Box::new(file)));
                }
                RedirKind::StderrAppend => {
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&redir.target)
                        .unwrap();
                    old_stderr = Some(std::mem::replace(&mut ctx.stderr, Box::new(file)));
                }
                RedirKind::Stdin => {
                    let file = File::open(&redir.target).unwrap();
                    old_stdin = Some(std::mem::replace(&mut ctx.stdin, Box::new(file)));
                }
            }
        }
        let cmd_name = &parsed.argv[0];
        let args: Vec<&str> = parsed.argv.iter().skip(1).map(|s| s.as_str()).collect();
        if let Some(cmd) = self.commands.get(cmd_name) {
            cmd.execute(&args, ctx);
        } else if let Some(_) = find_in_path(cmd_name) {
            match ProcCommand::new(cmd_name)
                .args(&args)
                .spawn()
                .and_then(|mut child| child.wait())
            {
                Ok(_status) => {
                    //writeln!(ctx.stdout, "{}", status).ok();
                }
                Err(e) => {
                    writeln!(ctx.stdout, "Failed to run {}: {}", cmd_name, e).ok();
                }
            }
        } else {
            writeln!(ctx.stdout, "{}: not found", cmd_name).ok();
        }
        if let Some(x) = old_stdin {
            ctx.stdin = x;
        }
        if let Some(x) = old_stdout {
            ctx.stdout = x;
        }
        if let Some(x) = old_stderr {
            ctx.stderr = x;
        }
        true
    }

    pub fn execute(&self, input: &str, ctx: &mut CommandContext) -> bool {
        let tokens = match ArgvTokenizer::tokenize(input) {
            Ok(v) => v,
            Err(e) => {
                writeln!(ctx.stderr, "tokenization error: {}", e).ok();
                return true;
            }
        };
        let pc = match parse_command(&tokens) {
            Ok(v) => v,
            Err(e) => {
                writeln!(ctx.stderr, "parse error: {}", e).ok();
                return true;
            }
        };
        if pc.argv.is_empty() {
            return true;
        }
        return self.execute_parsed(pc, ctx);
    }
}
