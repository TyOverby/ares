extern crate ares;
use ares::AresError::*;
use std::vec::Vec;

#[macro_use]
mod util;

#[test]
fn basic() {
    eval_ok!("((lambda () 1))", 1);
    eval_ok!("((lambda (a) (+ a 2)) 3)", 5);
   eval_ok!("((lambda (a b) (+ a b)) 3 4)", 7);
}

#[test]
fn nested() {
    eval_ok!(
        r"(((lambda (a b)
            (lambda (c d)
                (+ a b c d))) 1 2) 3 4)",
            10);
}

#[test]
fn multi_body() {
    eval_ok!(
        r"(((lambda (a b)
            5
            (lambda (c d)
                (+ a b c d))) 1 2) 3 4)",
        10);
}

#[test]
fn recursive() {
    eval_ok!(
        r"(define sum
              (lambda (s)
                  (if (= s 1)
                      1
                      (+ s (sum (- s 1))))))
          (sum 4)", 10);
}

#[test]
fn list_params() {
    eval_ok!("((lambda l l) 1 2 3)", vec![1, 2, 3]);
}

#[test]
fn rest_params() {
    /*
    eval_ok!("((lambda (. rest) rest) 1 2 3)", vec![1, 2, 3]);
    */
    eval_ok!("((lambda (x . rest) rest) 1 2 3)", vec![2, 3]);
    /*
    eval_ok!("((lambda (x y . rest) rest) 1 2 3)", vec![3]);
    eval_ok!("((lambda (x y z . rest) rest) 1 2 3)", Vec::<ares::Value>::new());
    eval_err!("((lambda (x y z . rest) rest) 1 2)", UnexpectedArity{..});
    eval_ok!("((lambda (x y . rest) (concat [x y] rest)) 1 2 3)", vec![1, 2, 3]);
    eval_err!("(lambda (x y . rest . rest2) rest)", UnexpectedArgsList(..));
    eval_err!("(lambda (x y . rest rest2) rest)", UnexpectedArgsList(..));
    eval_err!("(lambda (x y . .) rest)", UnexpectedArgsList(..));
    */
}
