extern crate ares;

#[macro_use]
mod util;

#[test]
fn symbol_quote() {
    eval_ok!("(quote 5)", 5);
    eval_ok!("(quote a)", ares::Value::symbol("a"));
    eval_ok!("'a", ares::Value::symbol("a"));
}

#[test]
fn list_quote() {
    eval_ok!("(quote (1 2 3))", ares::Value::list(vec![1.into(), 2.into(), 3.into()]));
}

