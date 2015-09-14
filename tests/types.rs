extern crate ares;

mod util;

use std::rc::Rc;

use ares::*;
use util::*;

#[test]
fn basic_if() {
    assert_eq!(e("5"), Value::Int(5));
    assert_eq!(e("-5"), Value::Int(-5));
    assert_eq!(e("5.0"), Value::Float(5.0));
    assert_eq!(e("-5.0"), Value::Float(-5.0));
    assert_eq!(e("true"), Value::Bool(true));
    assert_eq!(e("false"), Value::Bool(false));
    assert_eq!(e("\"foobar\""), Value::String(Rc::new("foobar".to_string())))
}
