extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic_let() {
    eval_ok!("(let (a 5) a)", 5);
    eval_ok!("(let () 5)", 5);
    eval_ok!("(define x 10) (let (x 4) x)", 4);
    eval_ok!("(define x 10) (let (x 4) (set x 15) x)", 15);
    eval_ok!("(let (x 4 x 5) x)", 5);
    eval_ok!("(let (x 4 x 5) x)", 5);
    eval_ok!("(let (x 4 y 5) (+ x y))", 9);
    eval_ok!("(let (x 4 y (+ x 1)) y)", 5);
}
