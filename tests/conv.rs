extern crate ares;

mod util;

use ares::*;
use util::*;

#[test]
fn convert_to_int() {
    assert_eq!(e("(->int 5)"), Value::Int(5));
    assert_eq!(e("(->int 1.2)"), Value::Int(1));
    assert_eq!(e("(->int \"10\")"), Value::Int(10));
    assert_eq!(e("(->int true)"), Value::Int(1));
    assert_eq!(e("(->int false)"), Value::Int(0));
}
