extern crate ares;
use ares::AresError;

#[macro_use]
mod util;

#[test]
fn basic_if() {
    eval_ok!("(if (= 1 1) 5 6)", 5);
    eval_ok!("(if (= 1 2) 5 6)", 6);
}

#[test]
fn basic_cond() {
    eval_ok!("(cond true 1)", 1);
    eval_ok!("(cond false 1 true 2)", 2);
    eval_ok!("(cond false 1 true 2 false 3)", 2);
    eval_ok!("(cond false 1 false 2 true 3)", 3);
    eval_err!("(cond false 1 false 2 false 3)", AresError::UnhandledCond);
    eval_ok!(
"(define even (lambda (x) (= 0 (% x 2))))
 (cond false 1 (even 2) 2 false 3)", 2);
}

#[test]
fn basic_switch() {
    eval_ok!("(define t (lambda (x) true)) (switch 5 t 1)", 1);
    eval_ok!(
"(switch 10
        string? 1
        bool? 2
        int? 3)", 3);
}
