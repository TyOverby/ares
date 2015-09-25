extern crate ares;

#[macro_use]
mod util;

#[test]
fn test_and() {
    eval_ok!("(and)", true);
    eval_ok!("(and true)", true);
    eval_ok!("(and false)", false);
    eval_ok!("(and true false)", false);
    eval_ok!("(and true true)", true);
    eval_ok!("(and true true false)", false);
}

#[test]
fn test_or() {
    eval_ok!("(or)", false);
    eval_ok!("(or true)", true);
    eval_ok!("(or false true)", true);
    eval_ok!("(or true false)", true);
    eval_ok!("(or false false)", false);
    eval_ok!("(or false)", false);
}

#[test]
fn test_xor() {
    eval_ok!("(xor)", false);
    eval_ok!("(xor true)", false);
    eval_ok!("(xor false)", false);
    eval_ok!("(xor true false)", true);
    eval_ok!("(xor true true false)", true);
    eval_ok!("(xor true false false)", true);
}

#[test]
fn test_shortcircuit() {
    eval_ok!(
        "(define x true)
         (define setter (lambda () (set! x false)))
         (and false (setter))
         x", true);
    eval_ok!(
        "(define x true)
         (define setter (lambda () (set! x false)))
         (or true (setter))
         x", true);
    eval_ok!(
        "(define x true)
         (define setter (lambda () (set! x false)))
         (xor true false (setter))
         x", true);
}
