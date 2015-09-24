extern crate ares;

#[macro_use]
mod util;

#[test]
fn build_list() {
    eval_ok!("(build-list (lambda (push) 1))", Vec::<i64>::new());
    eval_ok!("(build-list (lambda (push) (push 1 2 3)))", vec![1, 2, 3]);
    eval_ok!("(build-list (lambda (push) (push 1)))", vec![1]);
    eval_ok!("(build-list (lambda (push) (push 1) (push 2) (push 3)))", vec![1, 2, 3]);
}

#[test]
fn to_list() {
    eval_ok!("(list 1 2 3)", vec![1, 2, 3]);
}

#[test]
fn test_map() {
    eval_ok!("(map (list 1 2 3) (lambda (x) (+ x 1)))", vec![2, 3, 4]);
    eval_ok!("(map '(1 2 3) (lambda (x) (+ x 1)))", vec![2, 3, 4]);
}
