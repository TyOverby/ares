use std::fmt;
use std::char;
use std::str::CharIndices;
use std::iter::Peekable;
use parse::errors::ParseError;
use parse::errors::ParseError::*;

#[derive(Debug, Copy, Clone)]
pub struct Position(pub usize, pub usize);

impl<'a> fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.0, self.1)
    }
}

impl Position {
    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.0 += 1;
            self.1 = 0;
        } else {
            self.1 += 1;
        }
    }
    pub fn next(&self) -> Position {
        Position(self.0, self.1 + 1)
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Open(Open),
    Close(Close),
    FormLike(FormLike),
    String(String),
    Number(String),
    Symbol(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Open { LParen, LBrace, LBracket }
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Close { RParen, RBrace, RBracket }

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FormLike { Quote, QuasiQuote, Unquote, UnquoteSplicing }

impl FormLike {
    #[inline]
    pub fn form_name(&self) -> &'static str {
        use self::FormLike::*;
        match self {
            &Quote => "quote",
            &QuasiQuote => "quasiquote",
            &Unquote => "unquote",
            &UnquoteSplicing => "unquote-splicing"
        }
    }
}

use self::Open::*;
use self::Close::*;

impl Open {
    #[inline]
    pub fn closed_by(&self) -> Close {
        match self {
            &LParen => RParen,
            &LBrace => RBrace,
            &LBracket => RBracket
        }
    }
}

impl Close {
    #[inline]
    pub fn to_char(&self) -> char {
        match self {
            &RParen => ')',
            &RBrace => '}',
            &RBracket => ']'
        }
    }
}

pub struct Token {
    pub tt: TokenType,
    pub start: Position,
    pub end: Position
}

impl Token {
    pub fn new(t: TokenType, start: Position, end: Position) -> Token {
        Token { tt: t, start: start, end: end}
    }

    pub fn new_delim(c: char, start: Position) -> Option<Token> {
        use self::TokenType::{Open, Close};
        if let Some(tt) = match c {
            '(' => Some(Open(LParen)),
            ')' => Some(Close(RParen)),
            '[' => Some(Open(LBracket)),
            ']' => Some(Close(RBracket)),
            '{' => Some(Open(LBrace)),
            '}' => Some(Close(RBrace)),
            _ => None
        } {
            Some(Token { tt: tt, start: start, end: start.next() })
        } else {
            None
        }
    }
}

struct CharIndicesPos<'a> {
    pos: Position,
    iter: CharIndices<'a>
}

impl<'a> CharIndicesPos<'a> {
    pub fn new(p: Position, i: CharIndices<'a>) -> CharIndicesPos<'a> {
        CharIndicesPos { pos: p, iter: i }
    }
}

impl<'a> Iterator for CharIndicesPos<'a> {
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

pub struct TokenIter<'a>
{
    input: &'a str,
    iter: Peekable<CharIndicesPos<'a>>,
}

impl<'a> Iterator for TokenIter<'a>
{
    type Item = Result<Token, ParseError>;
    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        use self::TokenType::*;
        use self::FormLike::*;
        self.skip_ws();
        if let Some((start, curchar, pos)) = self.iter.next() {
            match curchar {
                '\'' => Some(Ok(Token::new(FormLike(Quote), pos, pos.next()))),
                '`' => Some(Ok(Token::new(FormLike(QuasiQuote), pos, pos.next()))),
                '~' => {
                    let unquote = Some(Ok(Token::new(FormLike(Unquote), pos, pos.next())));
                    if let Some(&(_, c, nextpos)) = self.iter.peek() {
                        if c == '@' {
                            self.iter.next();
                            Some(Ok(Token::new(FormLike(UnquoteSplicing), pos, nextpos.next())))
                        } else {
                            unquote
                        }
                    } else {
                        unquote
                    }
                },
                c if is_symbol_start_c(c) => Some(self.read_symbol(c, start, pos)),
                c if c.is_digit(10) => Some(self.read_number(start, pos)),
                '(' | ')' | '[' | ']' | '{' | '}' => Some(Ok(Token::new_delim(curchar, pos).unwrap())),
                '"' => Some(self.read_string(start + 1, pos)),
                _ => None
            }
        } else {
            None
        }
    }
}

macro_rules! delimcheck {
    ($c:expr, $pos: expr, $starting_pos: expr, $parsing: expr) => ({
        let c = $c;
        let spos = $starting_pos;
        if !is_delimiter_c(c) {
            return Err(
                UnexpectedChar(c,
                               $pos,
                               format!("while parsing a {} starting at {}",
                                       $parsing, spos)))
        }
    })
}

impl<'a> TokenIter<'a>
{
    pub fn new(s: &'a str) -> TokenIter<'a> {
        let iter = CharIndicesPos::new(Position(1, 0), s.char_indices());
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
            0 => Err(UnterminatedString(string_start)),
            l if l > 8 => Err(BadEscape(escape_start, self.input[start..chars[8].0].into())),
            l => {
                if chars[0].1 != '{' ||
                    !(chars.iter().skip(1).take(l-1)
                      .map(|&(_,c,_)| c).all(|c| c.is_digit(16))) {
                    Err(BadEscape(escape_start, self.input[start..brace+1].into()))
                } else {
                    let ival = chars
                        .iter()
                        .skip(1)
                        .take(l-1).fold(0, |acc, &(_, c, _)|
                                        acc * 16 + (c as u32 - '0' as u32));
                    char::from_u32(ival)
                            .ok_or(BadEscape(escape_start, self.input[start..brace+1].into()))
                }
            }
        }
    }

    fn read_x_escape<'b>(&'b mut self, start: usize, escape_start: Position, string_start: Position)
                         -> Result<char, ParseError> {
        // hand-rolled version of self.iter.take(2).collect()
        let v : Vec<_> = self.iter.next().map_or(vec![], (|x| self.iter.next().map_or(vec![x], |y| vec![x,y])));
        if v.len() < 2 {
            Err(UnterminatedString(string_start))
        } else {
            let c1 = v[0].1;
            let (end_index, c2, _) = v[1];
            if c1 > '7' || c1 < '0' {
                Err(BadEscape(escape_start, self.input[start..end_index].into()))
            } else {
                match c2 {
                    '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' => {
                        let zero = '0' as u32;
                        let ival = (c1 as u32 - zero) * 16 + (c2 as u32 - zero);
                        char::from_u32(ival)
                            .ok_or(BadEscape(escape_start, self.input[start..end_index].into()))
                    },
                    _ => Err(BadEscape(escape_start, self.input[start..end_index+1].into()))
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
                _ => Err(BadEscape(escape_start, self.input[start..end+1].into()))
            }
        } else {
            Err(UnterminatedString(string_start))
        }
    }

    fn read_string<'b>(&'b mut self, start: usize, startpos: Position) -> Result<Token, ParseError> {
        let mut start = Some(start);
        let mut string = String::new();
        let endpos;
        loop {
            let next = self.iter.next();
            match next {
                None => return Err(UnterminatedString(startpos)),
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
        Ok(Token::new(TokenType::String(string), startpos, endpos))
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
        Ok(Token::new(TokenType::Number(self.input[start..stop].into()), startpos, endpos))
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
                return Ok(Token::new(TokenType::Symbol(self.input[start..j].into()), startpos, pos))
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
        Ok(Token::new(TokenType::Symbol(self.input[start..stop].into()), startpos, endpos))
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
    (c.is_alphanumeric() || (c >= '*' && c <= '~') || c == '!' ||
        (c >= '#' && c <= '\'')) && !is_delimiter_c(c)
}

#[inline]
fn is_symbol_start_c(c: char) -> bool {
    is_symbol_c(c) && !c.is_numeric() && c != '\''
}
