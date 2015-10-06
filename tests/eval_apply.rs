extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_if() {
    eval_ok!("(eval (list + 1 2 3))", 6);
    eval_ok!("(apply + (list 1 2 3))", 6);
}
