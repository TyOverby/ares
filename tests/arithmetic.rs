extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_addition() {
    eval_ok!("(+)", 0);
    eval_ok!("(+ 1)", 1);
    eval_ok!("(+ 1 2)", 3);
    eval_ok!("(+ 1 (+ 2 3))", 6);
}

#[test]
fn basic_subtraction() {
    // eval_ok!("(-)", Value::Int(0)); // should fail
    eval_ok!("(- 1)", -1);
    eval_ok!("(- 1 2)", -1);
    eval_ok!("(- 1 (- 2 3))", 2);
}

#[test]
fn basic_multiplication() {
    eval_ok!("(*)", 1);
    eval_ok!("(* 2)", 2);
    eval_ok!("(* 2 2)", 4);
    eval_ok!("(* 2 (* 2 3))", 12);
}

#[test]
fn basic_division() {
    // eval_ok!("(/)", Value::Int(1)); // should fail
    eval_ok!("(/ 2)", 2);
    eval_ok!("(/ 4 2)", 2);
    eval_ok!("(/ 8 (/ 4 2))", 4);
}

#[test]
fn wrapping_int_arith() {
    eval_ok!(&format!("(+ {0} {0})", ::std::i64::MAX));
    eval_ok!(&format!("(- {0} {0})", ::std::i64::MAX));
    eval_ok!(&format!("(* {0} {0})", ::std::i64::MAX));
    eval_ok!(&format!("(/ {0} {0})", ::std::i64::MAX));
}
