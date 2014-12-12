#![experimental]

struct Lexer<'a> {
    input: Box<Buffer + 'a>
}

impl<'a> Lexer<'a> {
    fn new(input: Box<Buffer + 'a>) -> Lexer<'a> {
        Lexer { input: input }
    }
}

impl<'a> Iterator<String> for Lexer<'a> {
    fn next(&mut self) -> Option<String> {
        match self.input.read_char() {
            Ok(ch) => Some(ch.to_string()),
            Err(_) => None
        }
    }
}

#[test]
fn test_lex() {
    let string = "Hello, world!".to_string();

    let reader = std::io::MemReader::new(string.into_bytes());
    let mut lexer = Lexer::new(box reader);

    for token in lexer {
        println!("{}", token);
    }
}
