use crate::cli::parser::tokenize::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum RedirKind {
    StdoutTruncate, // >
    StdoutAppend,   // >>
    StderrTruncate, // 2>
    StderrAppend,   // 2>>
    Stdin,          // <
}

pub struct Redirection {
    pub kind: RedirKind,
    pub target: String,
}

pub struct ParsedCommand {
    pub argv: Vec<String>,
    pub redirects: Vec<Redirection>,
}

pub fn parse_command(tokens: &[Token]) -> Result<ParsedCommand, String> {
    let mut argv = vec![];
    let mut redirects = vec![];

    let mut iter = tokens.iter().peekable();

    while let Some(tok) = iter.next() {
        match tok {
            Token::Word(w) => argv.push(w.clone()),
            Token::RedirectOut => {
                let target = match iter.next() {
                    Some(Token::Word(w)) => w.clone(),
                    _ => return Err("expected filename after '>'".into()),
                };
                redirects.push(Redirection {
                    kind: RedirKind::StdoutTruncate,
                    target,
                });
            }
            Token::RedirectOutAppend => {
                let target = match iter.next() {
                    Some(Token::Word(w)) => w.clone(),
                    _ => return Err("expected filename after '>>'".into()),
                };
                redirects.push(Redirection {
                    kind: RedirKind::StdoutAppend,
                    target,
                });
            }
            Token::RedirectErr => {
                let target = match iter.next() {
                    Some(Token::Word(w)) => w.clone(),
                    _ => return Err("expected filename after '2>'".into()),
                };
                redirects.push(Redirection {
                    kind: RedirKind::StderrTruncate,
                    target,
                });
            }
            Token::RedirectErrAppend => {
                let target = match iter.next() {
                    Some(Token::Word(w)) => w.clone(),
                    _ => return Err("expected filename after '2>>'".into()),
                };
                redirects.push(Redirection {
                    kind: RedirKind::StderrAppend,
                    target,
                });
            }
            Token::RedirectIn => {
                let target = match iter.next() {
                    Some(Token::Word(w)) => w.clone(),
                    _ => return Err("expected filename after '<'".into()),
                };
                redirects.push(Redirection {
                    kind: RedirKind::Stdin,
                    target,
                });
            }
        }
    }
    Ok(ParsedCommand { argv, redirects })
}
