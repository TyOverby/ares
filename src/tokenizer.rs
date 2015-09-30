// Based on Norvig's lisp interpreter
use std::rc::Rc;
use std::error::Error;
use std::fmt;
use std::str::CharIndices;
use std::iter::Peekable;
use std::char;
use ::Value;

#[derive(Debug, Clone)]
enum TokenType {
    Open(Open),
    Close(Close),
    Quote,
    String(String),
    Number(String),
    Symbol(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Open { LParen, LBrace, LBracket }
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Close { RParen, RBrace, RBracket }

use tokenizer::Open::*;
use tokenizer::Close::*;

impl Open {
    #[inline]
    fn closed_by(&self) -> Close {
        match self {
            &LParen => RParen,
            &LBrace => RBrace,
            &LBracket => RBracket
        }
    }
}

impl Close {
    #[inline]
    fn to_char(&self) -> char {
        match self {
            &RParen => ')',
            &RBrace => '}',
            &RBracket => ']'
        }
    }
}

struct Token {
    tt: TokenType,
    start: Position,
    end: Option<Position>
}

impl Token {
    fn new(t: TokenType, start: Position, end: Option<Position>) -> Token {
        Token { tt: t, start: start, end: end}
    }
}

struct TokenIter<'a>
{
    input: &'a str,
    iter: Peekable<CharIndicesLineCol<'a>>,
}

struct CharIndicesLineCol<'a> {
    pos: Position,
    iter: CharIndices<'a>
}

#[derive(Debug, Copy, Clone)]
pub struct Position(usize, usize);

impl<'a> fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.0, self.1)
    }
}

impl Position {
    fn advance(&mut self, c: char) {
        if c == '\n' {
            self.0 += 1;
            self.1 = 0;
        } else {
            self.1 += 1;
        }
    }
    fn next(&self) -> Position {
        Position(self.0, self.1 + 1)
    }
}


#[derive(Debug)]
enum ParseError_ {
    UnexpectedChar(char, Position, String),
    UnterminatedString(Position),
    ConversionError(String, Box<Error>),
    BadEscape(Position, String),
    MissingRightDelimiter(Close),
    ExtraRightDelimiter(Close, Position),
}

#[derive(Debug)]
pub struct ParseError(ParseError_);

impl<'a> Iterator for CharIndicesLineCol<'a> {
    type Item = (usize, char, Position);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some((i, c)) => {
                self.pos.advance(c);
                Some((i, c, self.pos))
            }
        }
    }
}

macro_rules! delimcheck {
    ($c:expr, $pos: expr, $starting_pos: expr, $parsing: expr) => ({
        let c = $c;
        let spos = $starting_pos;
        if !is_delimiter_c(c) {
            return parse_error(
                UnexpectedChar(c,
                               $pos,
                               format!("while parsing a {} starting at line {}, column {}",
                                       $parsing, spos.0, spos.1)))
        }
    })
}

impl<'a> Iterator for TokenIter<'a>
{
    type Item = Result<Token, ParseError>;
    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        use tokenizer::TokenType::*;
        self.skip_ws();
        if let Some((start, curchar, pos)) = self.iter.next() {
            match curchar {
                '\'' => Some(Ok(Token::new(Quote, pos, None))),
                c if is_symbol_start_c(c) => Some(self.read_symbol(c, start, pos)),
                c if c.is_digit(10) => Some(self.read_number(start, pos)),
                '(' => Some(Ok(Token::new(Open(LParen), pos, None))),
                ')' => Some(Ok(Token::new(Close(RParen), pos, None))),
                '[' => Some(Ok(Token::new(Open(LBracket), pos, None))),
                ']' => Some(Ok(Token::new(Close(RBracket), pos, None))),
                '{' => Some(Ok(Token::new(Open(LBrace), pos, None))),
                '}' => Some(Ok(Token::new(Close(RBrace), pos, None))),
                '"' => Some(self.read_string(start + 1, pos)),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'a> TokenIter<'a>
{
    fn new(s: &'a str) -> TokenIter<'a> {
        let iter = CharIndicesLineCol { pos: Position(1, 0), iter: s.char_indices() };
        TokenIter { input: s, iter: iter.peekable() }
    }

    fn take_until<'b, F>(&'b mut self, f: F) -> (Vec<(usize, char, Position)>, Option<usize>)
        where F: Fn(char) -> bool
    {
        let mut v = vec![];
        let mut end = None;
        for (j, c, pos) in &mut self.iter {
            if !f(c) {
                v.push((j, c, pos))
            } else {
                end = Some(j);
                break;
            }
        }
        (v, end)
    }

    fn skip_ws<'b>(&'b mut self) {
        while self.iter.peek().map_or(false, |&(_, c, _)| c.is_whitespace()) {
            self.iter.next();
        }
    }

    fn read_u_escape<'b>(&'b mut self, start: usize, escape_start: Position, string_start: Position)
                         -> Result<char, ParseError> {
        let (chars, brace) = self.take_until(|c| c == '}');
        let brace = brace.unwrap_or(self.input.len());
        match chars.len() {
            0 => parse_error(UnterminatedString(string_start)),
            l if l > 8 => parse_error(BadEscape(escape_start, self.input[start..chars[8].0].into())),
            l => {
                if chars[0].1 != '{' ||
                    !(chars.iter().skip(1).take(l-1)
                      .map(|&(_,c,_)| c).all(|c| c.is_digit(16))) {
                    parse_error(BadEscape(escape_start, self.input[start..brace+1].into()))
                } else {
                    let ival = chars
                        .iter()
                        .skip(1)
                        .take(l-1).fold(0, |acc, &(_, c, _)|
                                        acc * 16 + (c as u32 - '0' as u32));
                    char::from_u32(ival)
                            .ok_or(ParseError(BadEscape(escape_start, self.input[start..brace+1].into())))
                }
            }
        }
    }

    fn read_x_escape<'b>(&'b mut self, start: usize, escape_start: Position, string_start: Position)
                         -> Result<char, ParseError> {
        // hand-rolled version of self.iter.take(2).collect()
        let v : Vec<_> = self.iter.next().map_or(vec![], (|x| self.iter.next().map_or(vec![x], |y| vec![x,y])));
        if v.len() < 2 {
            parse_error(UnterminatedString(string_start))
        } else {
            let c1 = v[0].1;
            let (end_index, c2, _) = v[1];
            if c1 > '7' || c1 < '0' {
                parse_error(BadEscape(escape_start, self.input[start..end_index].into()))
            } else {
                match c2 {
                    '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' => {
                        let zero = '0' as u32;
                        let ival = (c1 as u32 - zero) * 16 + (c2 as u32 - zero);
                        char::from_u32(ival)
                            .ok_or(ParseError(BadEscape(escape_start, self.input[start..end_index].into())))
                    },
                    _ => parse_error(BadEscape(escape_start, self.input[start..end_index+1].into()))
                }
            }
        }
    }

    fn read_escape<'b>(&'b mut self, start: usize, escape_start: Position, string_start: Position)
                       -> Result<char, ParseError> {
        if let Some((end, c, _pos)) = self.iter.next() {
            match c {
                'x' => self.read_x_escape(start, escape_start, string_start),
                'u' => self.read_u_escape(start, escape_start, string_start),
                't' => Ok('\t'),
                'r' => Ok('\r'),
                '\'' => Ok('\''),
                '"' => Ok('"'),
                'n' => Ok('\n'),
                _ => parse_error(BadEscape(escape_start, self.input[start..end+1].into()))
            }
        } else {
            parse_error(UnterminatedString(string_start))
        }
    }

    fn read_string<'b>(&'b mut self, start: usize, startpos: Position) -> Result<Token, ParseError> {
        let mut start = Some(start);
        let mut string = String::new();
        let endpos;
        loop {
            let next = self.iter.next();
            match next {
                None => return parse_error(UnterminatedString(startpos)),
                Some((j, c, pos)) => {
                    if start.is_none() { start = Some(j); }
                    if c == '\\' {
                        string.push_str(&self.input[start.unwrap()..j]);
                        string.push(try!(self.read_escape(j, pos, startpos)));
                        start = None;
                    } else if c == '"' {
                        string.push_str(&self.input[start.unwrap()..j]);
                        endpos = pos;
                        break
                    }
                }
            }
        };
        if let Some(&(_, c, pos)) = self.iter.peek() {
            delimcheck!(c, pos, startpos, "string");
        };
        Ok(Token::new(TokenType::String(string), startpos, Some(endpos)))
    }

    fn read_number<'b>(&'b mut self, start: usize, startpos: Position) -> Result<Token, ParseError> {
        let stop;
        let mut endpos = startpos.next();
        loop {
            if let Some(&(j, c, pos)) = self.iter.peek() {
                if !is_number_c(c) {
                    stop = j;
                    delimcheck!(c, pos, startpos, "number");
                    endpos = pos;
                    break
                }
                self.iter.next();
            } else {
                stop = self.input.len();
                break;
            }
        }
        Ok(Token::new(TokenType::Number(self.input[start..stop].into()), startpos, Some(endpos)))
    }

    fn read_symbol<'b>(&'b mut self, symstart: char, start: usize, startpos: Position)
                   -> Result<Token, ParseError> {
        let stop;
        let mut endpos = startpos.next();
        if let Some(&(j, c, pos)) = self.iter.peek() {
            endpos = pos;
            if c.is_digit(10) && (symstart == '+' || symstart == '-') {
                self.iter.next();
                return self.read_number(start, startpos)
            } else if is_delimiter_c(c) {
                return Ok(Token::new(TokenType::Symbol(self.input[start..j].into()), startpos, Some(pos)))
            }
        }
        self.iter.next();
        loop {
            if let Some(&(j, c, pos)) = self.iter.peek() {
                endpos = pos;
                if !is_symbol_c(c) {
                    stop = j;
                    delimcheck!(c, pos, startpos, "symbol");
                    break
                }
                self.iter.next();
            } else {
                stop = self.input.len();
                break
            }
        }
        Ok(Token::new(TokenType::Symbol(self.input[start..stop].into()), startpos, Some(endpos)))
    }
}

#[inline]
fn is_delimiter_c(c: char) -> bool {
    c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']'
}

#[inline]
fn is_number_c(c: char) -> bool {
    c.is_digit(10) || c == '.'  || c == 'e' || c == 'E'
}


#[inline]
fn is_symbol_c(c: char) -> bool {
    c.is_alphanumeric() || (c >= '*' && c <= '~') || c == '!' ||
        (c >= '#' && c<= '\'')
}

#[inline]
fn is_symbol_start_c(c: char) -> bool {
    is_symbol_c(c) && !c.is_numeric() && c != '\''
}

#[inline]
fn parse_error<T>(p: ParseError_) -> Result<T, ParseError> {
    Err(ParseError(p))
}

impl<'a> fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Error for ParseError {
    fn description(&self) -> &str { self.0.description() }
}

use tokenizer::ParseError_::*;

impl fmt::Display for ParseError_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnexpectedChar(c, pos, ref while_doing) =>
                write!(f,
                       "Unexpected character {} at line {}, column {}, {}",
                       c, pos.0, pos.1, while_doing),
            UnterminatedString(pos) =>
                write!(f, "Unterminated string beginning at line {}, column {}", pos.0, pos.1),
            ConversionError(ref s, ref e) => {
                write!(f, "Could not convert {}: {}", s, e)
            },
            BadEscape(pos, ref s) =>
                write!(f, "Invalid escape sequence starting at line {}, column {}: {}",
                       pos.0, pos.1, s),
            MissingRightDelimiter(c) => write!(f, "Missing right delimiter {}", c.to_char()),
            ExtraRightDelimiter(c, pos) =>
                write!(f, "Extra right delimiter {} at line {}, column {}", c.to_char(), pos.0, pos.1)
        }
    }
}

impl Error for ParseError_ {
    fn description(&self) -> &str {
        match *self {
            UnexpectedChar(_, _, _) => "Unexpected character",
            UnterminatedString(_) => "Unterminated string",
            ConversionError(_, ref e) => e.description(),
            BadEscape(..) => "Bad escape sequence",
            MissingRightDelimiter(..) => "Missing right delimiter",
            ExtraRightDelimiter(..) => "Extra right delimiter"
        }
    }
}

fn one_expr<'a, 'b>(tok: Token, tok_stream: &'a mut TokenIter<'b>)
                     -> Result<Value, ParseError> {
    use tokenizer::TokenType::*;
    match tok.tt {
        Number(s) => Ok(try!(s.parse().map(Value::Int)
                                 .or_else(|_| s.parse().map(Value::Float))
                                 .map_err(|e| ParseError(ConversionError(s, Box::new(e)))))),
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
        Close(close)  => parse_error(ExtraRightDelimiter(close, tok.start)),
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
                    return parse_error(ExtraRightDelimiter(opener.closed_by(), tok.start))
                },
                _ => v.push(try!(one_expr(tok, tok_stream)))
            }
        } else {
            return parse_error(MissingRightDelimiter(opener.closed_by()))
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
