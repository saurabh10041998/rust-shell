#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Word(String),
    RedirectIn,        // <
    RedirectOut,       // >
    RedirectOutAppend, // >>
    RedirectErr,       // 2>
    RedirectErrAppend, // 2>>
}

pub struct ArgvTokenizer;

impl ArgvTokenizer {
    pub fn tokenize(line: &str) -> Result<Vec<Token>, String> {
        use std::borrow::Cow;

        #[derive(Copy, Clone, PartialEq)]
        enum Mode {
            Normal,
            InSingle,
            InDouble,
        }
        let mut mode = Mode::Normal;
        let mut out = vec![];
        let mut buf = String::new();
        let mut chars = line.chars().peekable();

        let flush = |out: &mut Vec<Token>, buf: &mut String| {
            if !buf.is_empty() {
                out.push(Token::Word(std::mem::take(buf)));
            }
        };

        while let Some(ch) = chars.next() {
            match mode {
                Mode::Normal => {
                    match ch {
                        '\'' => mode = Mode::InSingle,
                        '"' => mode = Mode::InDouble,
                        '\\' => {
                            if let Some(n) = chars.next() {
                                buf.push(n);
                            }
                        }
                        '2' => {
                            if let Some('>') = chars.peek().copied() {
                                chars.next(); // consume '>'
                                flush(&mut out, &mut buf);

                                // check for >>
                                if let Some('>') = chars.peek().copied() {
                                    chars.next(); // consume '>'
                                    out.push(Token::RedirectErrAppend);
                                } else {
                                    out.push(Token::RedirectErr);
                                }
                            } else {
                                buf.push('2');
                            }
                        }
                        '1' => {
                            if let Some('>') = chars.peek().copied() {
                                chars.next(); // consume '>'
                                flush(&mut out, &mut buf);

                                // check for >>
                                if let Some('>') = chars.peek().copied() {
                                    chars.next(); // consume '>'
                                    out.push(Token::RedirectOutAppend);
                                } else {
                                    out.push(Token::RedirectOut);
                                }
                            } else {
                                buf.push('1');
                            }
                        }
                        '>' => {
                            flush(&mut out, &mut buf);

                            if let Some('>') = chars.peek().copied() {
                                chars.next();
                                out.push(Token::RedirectOutAppend);
                            } else {
                                out.push(Token::RedirectOut);
                            }
                        }
                        '<' => {
                            flush(&mut out, &mut buf);
                            out.push(Token::RedirectIn);
                        }
                        c if c.is_whitespace() => flush(&mut out, &mut buf),
                        c => buf.push(c),
                    }
                }
                Mode::InSingle => match ch {
                    '\'' => mode = Mode::Normal,
                    c => buf.push(c),
                },
                Mode::InDouble => match ch {
                    '"' => mode = Mode::Normal,
                    '\\' => {
                        if let Some(n) = chars.next() {
                            let push: Cow<'static, str> = match n {
                                '"' => "\"".into(),
                                '`' => "`".into(),
                                '\\' => "\\".into(),
                                '$' => "$".into(),
                                other => {
                                    buf.push('\\');
                                    buf.push(other);
                                    continue;
                                }
                            };
                            buf.push_str(&push);
                        };
                    }
                    c => buf.push(c),
                },
            }
        }

        if mode != Mode::Normal {
            return Err("unmatched quote".into());
        }
        if !buf.is_empty() {
            out.push(Token::Word(buf));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(
            ArgvTokenizer::tokenize("ls -la").unwrap(),
            vec![
                Token::Word(String::from("ls")),
                Token::Word(String::from("-la")),
            ]
        );
    }
    #[test]
    fn single_quotes() {
        assert_eq!(
            ArgvTokenizer::tokenize("echo 'hello world'").unwrap(),
            vec![
                Token::Word(String::from("echo")),
                Token::Word(String::from("hello world"))
            ]
        )
    }
    #[test]
    fn redirects() {
        assert_eq!(
            ArgvTokenizer::tokenize("cat < infile > outfile").unwrap(),
            vec![
                Token::Word(String::from("cat")),
                Token::RedirectIn,
                Token::Word(String::from("infile")),
                Token::RedirectOut,
                Token::Word(String::from("outfile")),
            ]
        );
    }
    #[test]
    fn redirect_stderr() {
        assert_eq!(
            ArgvTokenizer::tokenize("2> err.txt ls").unwrap(),
            vec![
                Token::RedirectErr,
                Token::Word(String::from("err.txt")),
                Token::Word(String::from("ls"))
            ]
        )
    }
    #[test]
    fn redirect_stdoutappend() {
        assert_eq!(
            ArgvTokenizer::tokenize("ls >> out.txt").unwrap(),
            vec![
                Token::Word(String::from("ls")),
                Token::RedirectOutAppend,
                Token::Word(String::from("out.txt")),
            ]
        );
    }
}
