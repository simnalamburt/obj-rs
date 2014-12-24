use self::ParseErrorKind::*;

pub struct ParseError {
    pub kind: ParseErrorKind,
    pub desc: &'static str,
}

impl Copy for ParseError {}

pub enum ParseErrorKind {
    UnexpectedStatement,
    WrongNumberOfArguments,
}

impl Copy for ParseErrorKind {}

pub fn parse_error(kind: ParseErrorKind) -> ParseError {
    let desc = match kind {
        UnexpectedStatement => "Unexpected statement",
        WrongNumberOfArguments => "Wrong number of arguments",
    };
    ParseError {
        kind: kind,
        desc: desc,
    }
}
