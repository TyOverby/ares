extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_types() {
    // TODO: test 1-arg and 2-arg
    eval_ok!("(= 1 1)", true);
    eval_ok!("(= 2 2 2 2)", true);
    eval_ok!("(= 1 2)", false);
    eval_ok!("(= 1 1 2)", false);
}
