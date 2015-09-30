// Based on Norvig's lisp interpreter
use std::rc::Rc;
use ::Value;

mod errors;
pub mod tokens;

use parse::tokens::{TokenType, Token, Open, TokenIter};
pub use parse::errors::{ParseError};
use parse::errors::ParseError::*;

fn one_expr<'a, 'b>(tok: Token, tok_stream: &'a mut TokenIter<'b>)
                     -> Result<Value, ParseError> {
    use self::tokens::TokenType::*;
    match tok.tt {
        Number(s) => Ok(try!(s.parse().map(Value::Int)
                                 .or_else(|_| s.parse().map(Value::Float))
                                 .map_err(|e| ConversionError(s, Box::new(e))))),
        Symbol(s) => Ok(s.parse().map(Value::Bool)
                        .unwrap_or(Value::Ident(Rc::new(s)))),
        String(s)     => Ok(Value::String(Rc::new(s))),
        Quote         => Ok({
            let quoted = try!(parse_one_expr(tok_stream));
            Value::new_list(match quoted {
                None => vec![Value::new_ident("quote")],
                Some(v) => vec![Value::new_ident("quote"), v]
            })
        }),
        Close(close)  => Err(ExtraRightDelimiter(close, tok.start)),
        Open(open)    => Ok(Value::new_list(try!(parse_delimited(tok_stream, open))))
    }
}


fn parse_one_expr<'a, 'b>(tok_stream: &'a mut TokenIter<'b>) -> Result<Option<Value>, ParseError> {
    if let Some(tok) = tok_stream.next() {
        one_expr(try!(tok), tok_stream).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_delimited<'a, 'b>(tok_stream: &'a mut TokenIter<'b>, opener: Open)
                      -> Result<Vec<Value>, ParseError> {
    let mut v = vec![];
    loop {
        if let Some(tok_or_err) = tok_stream.next() {
            let tok = try!(tok_or_err);
            match tok.tt {
                TokenType::Close(close) => if close == opener.closed_by() {
                    return Ok(v)
                } else {
                    return Err(ExtraRightDelimiter(opener.closed_by(), tok.start))
                },
                _ => v.push(try!(one_expr(tok, tok_stream)))
            }
        } else {
            return Err(MissingRightDelimiter(opener.closed_by()))
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<Value>, ParseError> {
    let mut v = vec![];
    let mut tok_iter = TokenIter::new(input);
    while let Some(value) = try!(parse_one_expr(&mut tok_iter)) {
         v.push(value)
    };
    Ok(v)
}
