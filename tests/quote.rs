extern crate ares;

#[macro_use]
mod util;

#[test]
fn ident_quote() {
    eval_ok!("(quote 5)", 5);
    eval_ok!("(quote a)", ares::Value::new_ident("a"));
    eval_ok!("'a", ares::Value::new_ident("a"));
}

#[test]
fn list_quote() {
    eval_ok!("(quote (1 2 3))", ares::Value::new_list(vec![1.into(), 2.into(), 3.into()]));
}

