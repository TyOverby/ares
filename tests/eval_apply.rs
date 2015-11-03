extern crate ares;

#[macro_use]
mod util;

#[test]
fn eval() {
    eval_ok!("(eval (list + 1 2 3))", 6);
}

#[test]
fn apply() {
    eval_ok!("(apply + (list 1 2 3))", 6);
}
