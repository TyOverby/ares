extern crate ares;

use ares::{Context, Value};

#[macro_use]
mod util;

#[test]
fn simple_macros() {
    let mut ctx = Context::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);
    
    eval_ok!("(define-macro begin (lambda forms `((lambda () ~@forms))))
              (begin 4 5)", Value::Int(5));
 
    eval_ok!("(define-macro begin (lambda forms `((lambda () ~@forms))))
              (define-macro unless (lambda (cod then) `(if ~cod '() (begin ~@then))))
              (unless false (begin 4 5))", Value::Int(5));
}
