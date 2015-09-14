extern crate ares;

mod util;

use ares::*;
use util::*;

#[test]
fn basic_types() {
    // TODO: test 1-arg and 2-arg
    assert_eq!( e("(= 1 1)"), Value::Bool(true));
    assert_eq!( e("(= 2 2 2 2)"), Value::Bool(true));
    assert_eq!( e("(= 1 2)"), Value::Bool(false));
    assert_eq!( e("(= 1 1 2)"), Value::Bool(false));
}
