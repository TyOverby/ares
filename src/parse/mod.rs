// Based on Norvig's lisp interpreter
use std::rc::Rc;
use ::Value;

mod errors;
mod util;
pub mod tokens;

use parse::tokens::{TokenType, Token, Open, TokenIter};
pub use parse::errors::{ParseError};
use parse::errors::ParseError::*;

fn one_expr<'a, 'b>(tok: Token, tok_stream: &'a mut TokenIter<'b>)
                     -> Result<Value, ParseError> {
    use self::tokens::TokenType;
    match tok.tt {
        TokenType::Number(s) => Ok(try!(s.parse().map(Value::Int)
                                        .or_else(|_| s.parse().map(Value::Float))
                                        .map_err(|e| ConversionError(s, Box::new(e))))),
        TokenType::Symbol(s) => Ok(s.parse().map(Value::Bool)
                                   .unwrap_or(Value::Symbol(Rc::new(s)))),
        TokenType::String(s)     => Ok(Value::String(Rc::new(s))),
        TokenType::Quote         => Ok({
            let quoted = try!(parse_one_expr(tok_stream));
            Value::list(match quoted {
                None => vec![Value::symbol("quote")],
                Some(v) => vec![Value::symbol("quote"), v]
            })
        }),
        TokenType::Close(close)  => Err(ExtraRightDelimiter(close, tok.start)),
        TokenType::Open(open)    => {
            let mut values = try!(parse_delimited(tok_stream, open));
            match open {
                Open::LParen => Ok(Value::list(values)),
                Open::LBracket => if values.iter().all(util::immediate_value) {
                    let values = values.into_iter().map(util::unquote).collect();
                    Ok(Value::list(vec![Value::symbol("quote"), Value::list(values)]))
                } else {
                    values.insert(0, Value::symbol("list"));
                    Ok(Value::list(values))
                },
                Open::LBrace => {
                    if values.len() % 2 == 1 {
                        return Err(InvalidMapLiteral(tok.start))
                    }
                    if values.iter().all(util::immediate_value) {
                        let (keys, values) : (Vec<_>, _) = values.into_iter().enumerate().partition(|&(i, _)| i % 2 == 0);
                        if keys.iter().all(|&(_, ref k)| util::can_be_hash_key(k)) {
                            let m = keys.into_iter().map(|(_, k)| util::unquote(k)).zip(values.into_iter().map(|(_, v)| util::unquote(v))).collect();
                            Ok(Value::Map(Rc::new(m)))
                        } else {
                            Err(InvalidMapLiteral(tok.start))
                        }
                    } else {
                        values.insert(0, Value::symbol("hash-map"));
                        Ok(Value::list(values))
                    }
                }
            }
        }
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
