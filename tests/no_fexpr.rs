extern crate ares;
use ares::{Context, AresError, Value};

#[macro_use]
mod util;

#[test]
fn test_no_fexpr() {
    eval_err!("(define q quote)");
    eval_err!("(+ 1 set)");
    eval_err!("(define foobar (lambda (a b) (a b))) (foobar quote 5)");
}

#[test]
fn test_no_fexpr_from_api() {
    let mut ctx = Context::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);
    ctx.eval_str("(define foobar (lambda (a b) (a b)))").unwrap();

    let quote = ctx.get("quote").unwrap();
    let b = ctx.interner_mut().intern("b");
    let res = ctx.call_named("foobar", &vec![quote, 5.into()]);
    match res {
        Err(AresError::AstFunctionPass) => assert!(true),
        Ok(Value::Symbol(x)) if x == b => assert!(false, "got 'b"),
        other => assert!(false, "{:?}", other)
    }
}
