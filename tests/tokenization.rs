extern crate ares;
use std::error::Error;
use ares::tokenizer::{parse, ParseError};
use ares::Value;

macro_rules! parse_fail {
    ($prog: expr, $estr: expr) => ({
        let parsed = parse($prog);
        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(format!("{}", err), $estr)
    })
}

macro_rules! parse_ok {
    ($prog: expr) => ({
        let parsed = parse($prog);
        assert!(parsed.is_ok())
    });
    ($prog: expr, $v: expr) => ({
        let parsed = parse($prog);
        assert!(parsed.is_ok());
        let v = parsed.unwrap();
        assert_eq!(v[0], Value::from($v));
    });
}

#[test]
fn parentheses() {
    parse_fail!("(x y(", "Missing right parenthesis");
    parse_fail!("(x y) ()) z", "Extra right parenthesis at line 1, column 9");
    parse_fail!("(x (y (z) \"())))))\"", "Missing right parenthesis");
    parse_ok!("(x (y (z \"())))\")))");
}

#[test]
fn strings() {
    parse_fail!("x
(\" 
sdf", "Unterminated string beginning at line 2, column 2");
    parse_fail!("x \"\\", "Unterminated string beginning at line 1, column 3");
    parse_ok!("\"\\\"\"", "\"");
}



#[allow(overflowing_literals)]
#[test]
fn numbers() {
    parse_fail!("(+ 3 32.e.)", "Could not convert 32.e.: invalid float literal");
    parse_ok!("-500e400", -500e400);
    parse_ok!("-500e4", -500e4);
    parse_ok!("-5.123", -5.123);
    parse_ok!("1", 1);
    parse_ok!("+1", 1.0);
    parse_ok!("+1.0", 1.0);
}

#[test]
fn symbols()
{
    parse_ok!("(foo-bz! ?? *wo+mp* +foo)");
}
