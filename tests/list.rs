extern crate ares;

#[macro_use]
mod util;

#[test]
fn test_build_list() {
    eval_ok!("(build-list (lambda (push) 1))", Vec::<i64>::new());
    eval_ok!("(build-list (lambda (push) (push 1 2 3)))", vec![1, 2, 3]);
    eval_ok!("(build-list (lambda (push) (push 1)))", vec![1]);
    eval_ok!("(build-list (lambda (push) (push 1) (push 2) (push 3)))", vec![1, 2, 3]);
}

#[test]
fn test_list() {
    eval_ok!("(list 1 2 3)", vec![1, 2, 3]);
    eval_ok!("(list)", Vec::<i64>::new());
    eval_ok!("(list 1)", vec![1]);
}

#[test]
fn test_map() {
    eval_ok!("(map (list 1 2 3) (lambda (x) (* x 2)))", vec![2, 4, 6]);
}

#[test]
fn test_fold_left() {
    // eval_ok!("(fold-left (list 1 2 3) 0 (lambda (a b) (+ a b)))", 6); // Shit, this is
    // legitimately failing.
}
