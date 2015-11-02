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


#[test]
fn quasiquote() {
    let mut ctx = Context::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    assert_eq!(ctx.eval_str("(let (x 2) `(1 ~x))").unwrap(), ares::Value::list(vec![1.into(), 2.into()]));
    assert_eq!(ctx.eval_str("(let (x '(2 3) y 'a) `(1 ~@x x ~y))").unwrap(),
             v!(1.into(), 2.into(), 3.into(), s!("x", ctx), s!("a", ctx)));
    assert_eq!(ctx.eval_str("(let (x 1) `~x)").unwrap(), ares::Value::Int(1));
    assert_eq!(ctx.eval_str("(define unless (lambda (cond body)
                `(if ~cond () ((lambda () ~@body)))))
               (unless true '((+ 1 2)))").unwrap(),
             v![s!("if", ctx), true.into(), v![],
                v!(v!(s!("lambda", ctx),
                      v![], v![s!("+", ctx), 1.into(), 2.into()]))]);
}
