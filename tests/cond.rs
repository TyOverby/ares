extern crate ares;

mod util;

use ares::*;
use util::*;

#[test]
fn basic_if() {
    assert_eq!(e("(if (= 1 1) 5 6)"), Value::Int(5));
    assert_eq!(e("(if (= 1 2) 5 6)"), Value::Int(6));
}
