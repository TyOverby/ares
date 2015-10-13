extern crate ares;

#[macro_use]
mod util;

#[test]
fn test_no_fexpr() {
    eval_err!("(define q quote)");
    eval_err!("(+ 1 set)");
    eval_err!("(define foobar (lambda (a b) (a b))) (foobar quote 5)");
}
