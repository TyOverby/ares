extern crate rebar;

use rebar::*;

use std::rc::Rc;
use std::cell::RefCell;

fn basic_environment() -> Rc<RefCell<Environment>> {
    let mut env = Environment::new();
    env.set_function("+", stdlib::add_ints);
    env.set_function("+.", stdlib::add_floats);

    env.set_function("-", stdlib::sub_ints);
    env.set_function("-.", stdlib::sub_floats);

    env.set_function("*", stdlib::mul_ints);
    env.set_function("*.", stdlib::mul_floats);

    env.set_function("/", stdlib::div_ints);
    env.set_function("/.", stdlib::div_floats);
    Rc::new(RefCell::new(env))
}

fn e(program: &str) -> Value {
    eval(&parse(program), &basic_environment())
}

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
