extern crate ares;
use std::collections::HashMap;
use ares::parse;
use ares::Value;
use ares::intern::SymbolIntern;


#[macro_use]
mod util;


macro_rules! parse_fail {
    ($prog: expr, $estr: expr) => ({
        let mut interner = SymbolIntern::new();
        let parsed = parse($prog, &mut interner);
        assert!(parsed.is_err());
        let err = parsed.unwrap_err();
        assert_eq!(format!("{}", err), $estr)
    })
}

macro_rules! parse_ok {
    ($prog: expr) => ({
        let mut interner = SymbolIntern::new();
        let parsed = parse($prog, &mut interner);
        assert!(parsed.is_ok())
    });
    ($prog: expr, $v: expr) => ({
        let mut interner = SymbolIntern::new();
        let parsed = parse($prog, &mut interner);
        assert!(parsed.is_ok());
        let v = parsed.unwrap();
        assert_eq!(v[0], Value::from($v));
    });
    ($prog: expr, $v: expr, $interner: expr) => ({
        let parsed = parse($prog, $interner);
        assert!(parsed.is_ok());
        let v = parsed.unwrap();
        assert_eq!(v[0], Value::from($v));
    });
}

#[test]
fn parentheses() {
    parse_fail!("(x y(", "Missing right delimiter )");
    parse_fail!("(x y) ()) z", "Extra right delimiter ) at line 1, column 9");
    parse_fail!("(x (y (z) \"())))))\"", "Missing right delimiter )");
    parse_ok!("(x (y (z \"())))\")))");
}

#[test]
fn strings() {
    parse_fail!("x
(\"
sdf", "Unterminated string beginning at line 2, column 2");
    parse_fail!("x \"\\", "Unterminated string beginning at line 1, column 3");
    parse_fail!("x \"\\x2", "Unterminated string beginning at line 1, column 3");
    parse_fail!("x \"\\u", "Unterminated string beginning at line 1, column 3");
    parse_ok!("\"\\\"\"", "\"");
    parse_ok!("\"\\x22\"", "\"");
    parse_ok!("\"\\u{2764}\"", "❤");
    parse_ok!("\"fo\\no\"", "fo\no");
    parse_fail!("\"fo\\wo\"", "Invalid escape sequence starting at line 1, column 4: \\w");
    parse_fail!("\"\\x99\"", "Invalid escape sequence starting at line 1, column 2: \\x9");
    parse_fail!("\"z\\x1x\"", "Invalid escape sequence starting at line 1, column 3: \\x1x");
    parse_fail!("\"\\u{999999}\"", "Invalid escape sequence starting at line 1, column 2: \\u{999999}");
    parse_fail!("(->int \"10\"x 5)", "Unexpected character x at line 1, column 12, while parsing a string starting at line 1, column 8");
}

#[allow(overflowing_literals)]
#[test]
fn numbers() {
    parse_fail!("(+ 3 32.e.)", "Could not convert 32.e.: invalid float literal");
    parse_ok!("-500e400", -500e400);
    parse_ok!("-500e4", -500e4);
    // TODO: -5.123 is parsed as -5.1229999999999999
    // parse_ok!("-5.123", -5.123);
    parse_ok!("1", 1);

    parse_ok!("+1.0", 1.0);
    parse_fail!("(+ 3z)", "Unexpected character z at line 1, column 5, while parsing a number starting at line 1, column 4");
}

#[test]
fn symbols()
{
    parse_ok!("(foo-bz! ?? *wo+mp* +foo)");
}

#[test]
fn quote()
{
    parse_ok!("(foo '(1 2))");
    parse_ok!("(foo '(1 (2 3) \"df)\"))");
    // these are admittedly weird error messages.
    parse_fail!("(foo ')", "Extra right delimiter ) at line 1, column 7");
    parse_fail!("(foo '", "Missing right delimiter )");
}


#[test]
fn mapliteral()
{
    parse_fail!("{1 2 3}", "Map literal at line 1, column 1 is malformed");
    parse_fail!("{1 2 {} 4}", "Map literal at line 1, column 1 is malformed");
    parse_fail!("{1 2 [1 2] 4}", "Map literal at line 1, column 1 is malformed");
    parse_ok!("{}", HashMap::<Value, Value>::new());
    parse_ok!("{\"1\" 2 'a 4 1.0 {1 2}}");
}


#[test]
fn listliteral()
{

    parse_ok!("[1 2]");
    parse_ok!("[1 {1 2}]");

}

#[test]
fn has_expected_quotes() {
    let mut interner = SymbolIntern::new();
    parse_ok!("[1 'a]", Value::list(
            vec![Value::Symbol(interner.intern("quote")),
                 Value::list(vec![1.into(), Value::Symbol(interner.intern("a"))])]), &mut interner);
    parse_ok!("{'a 'b}", hashmap!(Value::Symbol(interner.intern("a")) => Value::Symbol(interner.intern("b"))), &mut interner);
}
