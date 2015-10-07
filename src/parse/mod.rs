// Based on Norvig's lisp interpreter
use std::rc::Rc;
use ::Value;
use ::intern::SymbolIntern;

mod errors;
mod util;
pub mod tokens;

use parse::tokens::{TokenType, Token, Open, TokenIter};
pub use parse::errors::{ParseError};
use parse::errors::ParseError::*;

fn one_expr<'a, 'b>(tok: Token, tok_stream: &'a mut TokenIter<'b>, interner: &mut SymbolIntern)
                     -> Result<Value, ParseError> {
    use self::tokens::TokenType;
    match tok.tt {
        TokenType::Number(s) => Ok(try!(s.parse().map(Value::Int)
                                        .or_else(|_| s.parse().map(Value::Float))
                                        .map_err(|e| ConversionError(s, Box::new(e))))),
        TokenType::Symbol(s) => Ok(s.parse().map(Value::Bool)
                                   .unwrap_or(Value::Symbol(interner.intern(s)))),
        TokenType::String(s)     => Ok(Value::String(Rc::new(s))),
        TokenType::FormLike(fl)  => Ok({
            let quoted = try!(parse_one_expr(tok_stream));
            let interned = Value::Symbol(interner.intern(fl.form_name()));
            Value::list(match quoted {
                None => vec![interned],
                Some(v) => vec![interned, v]
            })
        }),
        TokenType::Close(close)  => Err(ExtraRightDelimiter(close, tok.start)),
        TokenType::Open(open)    => {
            let mut values = try!(parse_delimited(tok_stream, open, interner));
            match open {
                Open::LParen => Ok(Value::list(values)),
                Open::LBracket => if values.iter().all(|a| util::immediate_value(a, interner)) {
                    let values = values.into_iter().map(util::unquote).collect();
                    Ok(Value::list(vec![Value::Symbol(interner.intern("quote")), Value::list(values)]))
                } else {
                    values.insert(0, Value::Symbol(interner.intern("list")));
                    Ok(Value::list(values))
                },
                Open::LBrace => {
                    if values.len() % 2 == 1 {
                        return Err(InvalidMapLiteral(tok.start))
                    }
                    if values.iter().all(|a| util::immediate_value(a, interner)) {
                        let (keys, values) : (Vec<_>, _) = values.into_iter().enumerate().partition(|&(i, _)| i % 2 == 0);
                        if keys.iter().all(|&(_, ref k)| util::can_be_hash_key(k, interner)) {
                            let m = keys.into_iter().map(|(_, k)| util::unquote(k)).zip(values.into_iter().map(|(_, v)| util::unquote(v))).collect();
                            Ok(Value::Map(Rc::new(m)))
                        } else {
                            Err(InvalidMapLiteral(tok.start))
                        }
                    } else {
                        values.insert(0, Value::Symbol(interner.intern("hash-map")));
                        Ok(Value::list(values))
                    }
                }
            }
        }
    }
}


fn parse_one_expr<'a, 'b>(tok_stream: &'a mut TokenIter<'b>, interner: &mut SymbolIntern)
-> Result<Option<Value>, ParseError>
{
    if let Some(tok) = tok_stream.next() {
        one_expr(try!(tok), tok_stream, interner).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_delimited<'a, 'b>(tok_stream: &'a mut TokenIter<'b>, opener: Open, interner: &mut SymbolIntern)
-> Result<Vec<Value>, ParseError>
{
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
                _ => v.push(try!(one_expr(tok, tok_stream, interner)))
            }
        } else {
            return Err(MissingRightDelimiter(opener.closed_by()))
        }
    }
}

pub fn parse(input: &str, interner: &mut SymbolIntern) -> Result<Vec<Value>, ParseError> {
    let mut v = vec![];
    let mut tok_iter = TokenIter::new(input);
    while let Some(value) = try!(parse_one_expr(&mut tok_iter, interner)) {
         v.push(value)
    };
    Ok(v)
}
