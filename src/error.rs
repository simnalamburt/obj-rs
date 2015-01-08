#[derive(Copy)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub desc: &'static str,
}

#[derive(Copy)]
pub enum ParseErrorKind {
    UnexpectedStatement,
    WrongNumberOfArguments,
}

pub fn parse_error(kind: ParseErrorKind) -> ParseError {
    let desc = match kind {
        ParseErrorKind::UnexpectedStatement => "Unexpected statement",
        ParseErrorKind::WrongNumberOfArguments => "Wrong number of arguments",
    };

    ParseError {
        kind: kind,
        desc: desc,
    }
}

macro_rules! error {
    ($kind:ident) => {
        return Some(parse_error(ParseErrorKind::$kind))
    }
}
