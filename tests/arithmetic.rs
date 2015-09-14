extern crate ares;

mod util;

use ares::*;
use util::*;

#[test]
fn basic_addition() {
    assert_eq!(e("(+)"), Value::Int(0));
    assert_eq!(e("(+ 1)"), Value::Int(1));
    assert_eq!(e("(+ 1 2)"), Value::Int(3));
    assert_eq!(e("(+ 1 (+ 2 3))"), Value::Int(6));
}

#[test]
fn basic_subtraction() {
    // assert_eq!(e("(-)", Value::Int(0)); // should fail
    assert_eq!(e("(- 1)"), Value::Int(-1));
    assert_eq!(e("(- 1 2)"), Value::Int(-1));
    assert_eq!(e("(- 1 (- 2 3))"), Value::Int(2));
}

#[test]
fn basic_multiplication() {
    assert_eq!(e("(*)"), Value::Int(1));
    assert_eq!(e("(* 2)"), Value::Int(2));
    assert_eq!(e("(* 2 2)"), Value::Int(4));
    assert_eq!(e("(* 2 (* 2 3))"), Value::Int(12));
}

#[test]
fn basic_division() {
    // assert_eq!(e("(/)"), Value::Int(1)); // should fail
    assert_eq!(e("(/ 2)"), Value::Int(2));
    assert_eq!(e("(/ 4 2)"), Value::Int(2));
    assert_eq!(e("(/ 8 (/ 4 2))"), Value::Int(4));
}
