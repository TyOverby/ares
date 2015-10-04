use std::error::Error;
use std::fmt;
use parse::tokens::{Position, Close};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char, Position, String),
    UnterminatedString(Position),
    ConversionError(String, Box<Error>),
    BadEscape(Position, String),
    MissingRightDelimiter(Close),
    ExtraRightDelimiter(Close, Position),
    InvalidMapLiteral(Position),
}

use self::ParseError::*;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnexpectedChar(c, pos, ref while_doing) =>
                write!(f,
                       "Unexpected character {} at {}, {}",
                       c, pos, while_doing),
            UnterminatedString(pos) =>
                write!(f, "Unterminated string beginning at {}", pos),
            ConversionError(ref s, ref e) => {
                write!(f, "Could not convert {}: {}", s, e)
            },
            BadEscape(pos, ref s) =>
                write!(f, "Invalid escape sequence starting at {}: {}", pos, s),
            MissingRightDelimiter(c) => write!(f, "Missing right delimiter {}", c.to_char()),
            ExtraRightDelimiter(c, pos) =>
                write!(f, "Extra right delimiter {} at {}", c.to_char(), pos),
            InvalidMapLiteral(pos) => write!(f, "Map literal at {} is malformed", pos),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            UnexpectedChar(_, _, _) => "Unexpected character",
            UnterminatedString(_) => "Unterminated string",
            ConversionError(_, ref e) => e.description(),
            BadEscape(..) => "Bad escape sequence",
            MissingRightDelimiter(..) => "Missing right delimiter",
            ExtraRightDelimiter(..) => "Extra right delimiter",
            InvalidMapLiteral(..) => "Map literals require an even number of elements",
        }
    }
}
