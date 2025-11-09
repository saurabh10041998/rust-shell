/// A simple POSIX-like argv spliter that understands:
/// - single quotes: 'literal'
/// - double quotes: "literal with escapes"
/// - backslash ecscapes outside quotes
///
/// It produces owned strings so the caller can then create &[&str] views

pub struct ArgvSplitter;

impl ArgvSplitter {
    pub fn split(line: &str) -> Result<Vec<String>, String> {
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

        while let Some(ch) = chars.next() {
            match mode {
                Mode::Normal => match ch {
                    '\'' => mode = Mode::InSingle,
                    '"' => mode = Mode::InDouble,
                    '\\' => {
                        if let Some(n) = chars.next() {
                            buf.push(n)
                        }
                    }
                    c if c.is_whitespace() => {
                        if !buf.is_empty() {
                            out.push(std::mem::take(&mut buf));
                        }
                    }
                    c => buf.push(c),
                },
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
                        }
                    }
                    c => buf.push(c),
                },
            }
        }

        if mode != Mode::Normal {
            return Err("unmatched quote".into());
        }
        if !buf.is_empty() {
            out.push(buf);
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
            ArgvSplitter::split("cd ls -la").unwrap(),
            vec!["cd", "ls", "-la"]
        );
    }

    #[test]
    fn single_quotes() {
        assert_eq!(
            ArgvSplitter::split("cd 'ls -la'").unwrap(),
            vec!["cd", "ls -la"]
        );
    }

    #[test]
    fn backslash_space() {
        assert_eq!(
            ArgvSplitter::split(r#"touch a\ b"#).unwrap(),
            vec!["touch", "a b"]
        )
    }

    #[test]
    fn double_quotes_escapes() {
        assert_eq!(
            ArgvSplitter::split(r#"echo "a b\" c""#).unwrap(),
            vec!["echo", r#"a b" c"#]
        )
    }

    #[test]
    fn unmatched_quotes() {
        assert!(ArgvSplitter::split("echo 'opps").is_err());
    }
}
