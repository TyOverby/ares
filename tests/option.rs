extern crate ares;

#[macro_use]
mod util;

#[test]
fn make_some() {
    eval_ok!("(some 5)", Some(5));
    eval_ok!("(some (some 5))", Some(Some(5)));
}

#[test]
fn make_none() {
    eval_ok!("(none)", None::<i64>);
}

#[test]
fn test_unwrap() {
    eval_ok!("(unwrap (some 5))", 5);
}
