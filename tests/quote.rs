extern crate ares;

use ares::{Context, Value};

#[macro_use]
mod util;

#[test]
fn symbol_quote() {
    let mut ctx = Context::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    assert_eq!(ctx.eval_str("(quote 5)").unwrap(), 5.into());
    assert_eq!(ctx.eval_str("(quote a)").unwrap(), Value::Symbol(ctx.interner_mut().intern("a")));
    assert_eq!(ctx.eval_str("'a").unwrap(), Value::Symbol(ctx.interner_mut().intern("a")));
}

#[test]
fn list_quote() {
    eval_ok!("(quote (1 2 3))", ares::Value::list(vec![1.into(), 2.into(), 3.into()]));
}

