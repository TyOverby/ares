extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_if() {
    eval_ok!("(if (= 1 1) 5 6)", 5);
    eval_ok!("(if (= 1 2) 5 6)", 6);
}
