use crate::error::{LoadError, LoadErrorKind, ObjError, ObjResult};
use std::io::{BufRead, Lines, Result};
use std::iter::Map;

fn strip_comment(mut line: String) -> String {
    if let Some(idx) = line.find('#') {
        line.truncate(idx)
    }
    line
}

#[test]
fn test_strip_commect() {
    macro_rules! t {
        ($input:expr => $output:expr) => {
            assert_eq!(strip_comment(String::from($input)), String::from($output));
        };
    }

    t!("Hello, world!" => "Hello, world!");
    t!("abc # def" => "abc ");
    t!("한글 # 한글" => "한글 ");
    t!("" => "");
}

type StrippedLines<T> = Map<Lines<T>, fn(Result<String>) -> Result<String>>;

#[derive(Debug)]
pub struct Lexer<T> {
    stripped_lines: StrippedLines<T>,
}

impl<T: BufRead> Lexer<T> {
    pub fn new(input: T) -> Self {
        Lexer {
            stripped_lines: input.lines().map(|result| result.map(strip_comment)),
        }
    }
}

impl<T: BufRead> Iterator for Lexer<T> {
    type Item = ObjResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if maybe_line has finished
        let maybe_line = self.stripped_lines.next()?;

        // Check if maybe_line has errored
        let line = match maybe_line {
            Err(e) => return Some(Err(ObjError::Io(e))),
            Ok(val) => val,
        };

        // Merge lines connected with backslashes
        let mut buffer = String::new();
        match line.strip_suffix('\\') {
            None => buffer.push_str(&line),
            Some(stripped) => {
                buffer.push_str(stripped);
                buffer.push(' ');

                // Search for the next lines
                loop {
                    let line;
                    match self.stripped_lines.next() {
                        None => {
                            return Some(Err(ObjError::Load(LoadError::new_internal(
                                LoadErrorKind::BackslashAtEOF,
                                "Expected a line, but met an EOF".to_string(),
                            ))));
                        }
                        Some(Err(e)) => return Some(Err(ObjError::Io(e))),
                        Some(Ok(val)) => line = val,
                    }
                    match line.strip_suffix('\\') {
                        Some(stripped) => {
                            buffer.push_str(stripped);
                            buffer.push(' ');
                        }
                        None => {
                            buffer.push_str(&line);
                            break;
                        }
                    }
                }
            }
        }

        Some(Ok(buffer))
    }
}

pub fn lex<T, F>(input: T, mut callback: F) -> ObjResult<()>
where
    T: BufRead,
    F: FnMut(&str, &[&str]) -> ObjResult<()>,
{
    for maybe_buffer in Lexer::new(input) {
        if let [stmt, ref args @ ..] = maybe_buffer?.split_whitespace().collect::<Vec<_>>()[..] {
            callback(stmt, args)?
        }
    }

    Ok(())
}

#[test]
fn test_lex() {
    let input = r#"
   statement0      arg0  arg1	arg2#argX   argX
statement1 arg0    arg1
# Comment
statement2 Hello, world!
bmat u  1       -3      3       -1      \
        0       3       -6      3       \
        0       0       3       -3      \
        0       0       0       1
bmat u  1       -3      3       -1      0       3       -6      3       \
        0       0       3       -3      0       0       0       1
bmat u  1       -3      3       -1      0       3       -6      3       0       0       3       -3      0       0       0       1
"#;

    assert!(
        lex(&mut input.as_bytes(), |stmt, args| {
            match stmt {
                "statement0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
                "statement1" => assert_eq!(args, ["arg0", "arg1"]),
                "statement2" => assert_eq!(args, ["Hello,", "world!"]),
                "bmat" => assert_eq!(
                    args,
                    [
                        "u", "1", "-3", "3", "-1", "0", "3", "-6", "3", "0", "0", "3", "-3", "0",
                        "0", "0", "1"
                    ]
                ),
                _ => panic!("Unit test failed"),
            }
            Ok(())
        })
        .is_ok()
    );
}
