extern crate ares;

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
