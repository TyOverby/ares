extern crate ares;

#[macro_use]
mod util;

#[test]
fn basic() {
    eval_ok!("(build-list (lambda (push) 1))", Vec::<i64>::new());
    eval_ok!("(build-list (lambda (push) (push 1)))", vec![1i64]);
    eval_ok!("(build-list (lambda (push) (push 1) (push 2) (push 3)))", vec![1i64, 2i64, 3i64]);
}
