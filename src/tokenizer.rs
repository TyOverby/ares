// Based on Norvig's lisp interpreter
use std::rc::Rc;
use std::error::Error;
use std::fmt;
use super::Value;

#[derive(Debug, Clone)]
enum Token<'a> {
    RParen,
    LParen,
    String(&'a str),
    Number(&'a str),
    Symbol(&'a str)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenState {
    String,
    Symbol,
    StringSkip,
    Number,
    Whitespace
}

type Position = (usize, usize);

#[derive(Debug)]
enum ParseError_<'a> {
    UnexpectedChar(char, Position, TokenState),
    UnterminatedString(Position),
    ConversionError(&'a str, Box<Error>),
    MissingRParen,
    ExtraRParen(Position)
}

#[derive(Debug)]
pub struct ParseError<'a>(ParseError_<'a>);

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str { self.0.description() }
}

impl<'a> fmt::Display for ParseError_<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError_::UnexpectedChar(c, (line, col), tokstate) => 
                write!(f,
                       "Unexpected character {} at line {}, column {}, token state {:?}",
                       c, line, col, tokstate),
            ParseError_::UnterminatedString((line, col)) =>
                write!(f, "Unterminated string beginning at line {}, column {}", line, col),
            ParseError_::ConversionError(ref s, ref e) => {
                write!(f, "Could not convert {}: {}", s, e)
            },
            ParseError_::MissingRParen => write!(f, "Missing right parenthesis"),
            ParseError_::ExtraRParen((line, col)) => write!(f, "Extra right parenthesis at line {}, column {}", line, col)
        }
    }
}

impl<'a> Error for ParseError_<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError_::UnexpectedChar(_, _, _) => "Unexpected character",
            ParseError_::UnterminatedString(_) => "Unterminated string",
            ParseError_::ConversionError(_, ref e) => e.description(),
            ParseError_::MissingRParen => "Missing right parenthesis",
            ParseError_::ExtraRParen(_) => "Extra right parenthesis"
        }
    }
}

fn parse_error<T>(p: ParseError_) -> Result<T, ParseError> {
    Err(ParseError(p))
}



fn escape_string(s: &str) -> String {
    let mut was_escaped = false;
    let mut string = String::with_capacity(s.len());
    for c in s.chars() {
        if was_escaped {
            string.push(match c {
                't' => '\t',
                'r' => '\r',
                'n' => '\n',
                '"' => '\"',
                _ => c
            });
            was_escaped = false;
        } else if c == '\\' {
            was_escaped = true;
        } else {
            string.push(c);
        }
    };
    string
}

fn tokenize(s: &str) -> Result<Vec<Token>, ParseError> {
    let mut col = 0;
    let mut line = 1;
    let mut i = 0;
    let mut nesting = 0;
    let mut tokenizing = TokenState::Whitespace;
    let mut tokens = vec![];
    let mut sym_start = None;
    let mut string_start_pos = (1, 1);
    for (j, c) in s.char_indices() {
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        if c.is_whitespace() || c == ')' || c == '(' {
            match tokenizing {
                TokenState::Symbol => { 
                    tokens.push(Token::Symbol(&s[i..j]));
                    tokenizing = TokenState::Whitespace;
                },
                TokenState::Number => {
                    tokens.push(Token::Number(&s[i..j]));
                    tokenizing = TokenState::Whitespace;
                },
                TokenState::StringSkip => tokenizing = TokenState::String,
                _ => ()
            }
            if tokenizing != TokenState::String {
                if c == '(' {
                    nesting += 1;
                    tokens.push(Token::LParen);
                } else if c == ')' {
                    nesting -= 1;
                    if nesting < 0 {
                        return parse_error(ParseError_::ExtraRParen((line, col)))
                    }
                    tokens.push(Token::RParen);
                }
            }
        } else { 
            match tokenizing {
                TokenState::Whitespace => {
                    if c == '"' {
                        i = j + 1;
                        string_start_pos = (line, col);
                        tokenizing = TokenState::String;
                    } else if c.is_digit(10) {
                        i = j;
                        tokenizing = TokenState::Number;
                    }  else {
                        tokenizing = TokenState::Symbol;
                        sym_start = Some(c);
                        i = j;
                    } 
                },
                TokenState::String => {
                    if c == '"' {
                        tokenizing = TokenState::Whitespace;
                        tokens.push(Token::String(&s[i..j]));
                    } else if c == '\\' {
                        tokenizing = TokenState::StringSkip
                    } else { () }
                },
                TokenState::StringSkip => tokenizing = TokenState::String,
                TokenState::Symbol => {
                    if i + 1 == j && (sym_start == Some('+') || sym_start == Some('-')) && c.is_digit(10) {
                        tokenizing = TokenState::Number;
                    } else if !(c.is_alphanumeric() || (c >= '*' && c <= '~') || c == '!' ||
                                (c >= '#' && c <= '\'')) {
                        return parse_error(ParseError_::UnexpectedChar(c, (line, col), tokenizing));
                    }
                },
                TokenState::Number => {
                    if !(c.is_digit(10) || c == 'e' || c == 'E' || c == '.') {
                        return parse_error(ParseError_::UnexpectedChar(c, (line, col), tokenizing))
                    }
                }
            } 
        }
    };
    match tokenizing {
        TokenState::String | TokenState::StringSkip => return parse_error(ParseError_::UnterminatedString(string_start_pos)),
        TokenState::Symbol => tokens.push(Token::Symbol(&s[i..s.len()])),
        TokenState::Number => tokens.push(Token::Number(&s[i..s.len()])),
        TokenState::Whitespace => ()
    };
    if nesting > 0 {
        return parse_error(ParseError_::MissingRParen)
    }
    Ok(tokens)
}

fn parse_tokens<'a, 'b: 'a, I>(i: &mut I) -> Result<Vec<Value>, ParseError<'b>>
    where I: Iterator<Item=&'a Token<'b>>
{
    let mut v = vec![];
    loop {
        let value = if let Some(token) = i.next() {
            match token {
                &Token::Number(ref s) => Some(try!(s.parse().map(Value::Int)
                                                   .or_else(|_| s.parse().map(Value::Float))
                                                   .map_err(|e| ParseError(ParseError_::ConversionError(s, Box::new(e)))))),
                &Token::Symbol(ref s) => Some(s.parse().map(Value::Bool)
                                              .unwrap_or(Value::Ident(Rc::new(s.to_string())))),
                &Token::String(ref s) => Some(Value::String(Rc::new(escape_string(s)))),
                &Token::RParen        => break,
                &Token::LParen        => None
            }
        } else { 
            break
        };
        match value {
            Some(value) => v.push(value),
            None => v.push(Value::List(Rc::new(try!(parse_tokens(i)))))
        }
    }
    Ok(v)
}

pub fn parse(input: &str) -> Result<Vec<Value>, ParseError> {
    let tokens = tokenize(input);
    tokens.and_then(|tokens| {  
        parse_tokens(&mut tokens.iter())
    })
}
