extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_set() {
    eval_ok!("(define x 5) (set x 10) x", 10);
    eval_ok!("(define f (lambda (a) (set a 3) a)) (f 5) ", 3);
    eval_ok!("(define f (lambda (a) (set a 3))) (f 5) ", 3);
}
