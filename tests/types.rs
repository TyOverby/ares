extern crate ares;

#[macro_use]
mod util;

#[test]
fn unit_types() {
    eval_ok!("5", 5);
    eval_ok!("-5", -5);
    eval_ok!("5.0", 5.0);
    eval_ok!("-5.0", -5.0);
    eval_ok!("true", true);
    eval_ok!("false", false);
    eval_ok!("\"foobar\"", "foobar");
}
