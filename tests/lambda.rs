extern crate rebar;

mod util;

use rebar::*;
use util::*;

#[test]
fn basic() {
    assert_eq!(e("((lambda () 1))"), Value::Int(1));
    assert_eq!(e("((lambda (a) (+ a 2)) 3)"), Value::Int(5));
    assert_eq!(e("((lambda (a b) (+ a b)) 3 4)"), Value::Int(7));
}

#[test]
fn nested() {
    assert_eq!(e(
r"(((lambda (a b)
    (lambda (c d)
        (+ a b c d))) 1 2) 3 4)"),
    Value::Int(10));
}

#[test]
fn multi_body() {
    assert_eq!(e(
r"(((lambda (a b)
    5
    (lambda (c d)
        (+ a b c d))) 1 2) 3 4)"),
               Value::Int(10));
}

/*
#[test]
fn recursive() {
    assert_e1(e(
r"(define sum (lambda ))"), Value::Int())
}
*/
