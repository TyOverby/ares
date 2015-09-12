extern crate rebar;

mod util;

use rebar::*;
use util::*;


#[test]
fn basic_define() {
    assert_eq!(
    e("(define x 5)
       (+ x 1)"), Value::Int(6));
}

#[test]
fn lambda_define() {
    assert_eq!(
    e("((lambda ()
            (define x 5)
            x))"), Value::Int(5));

    assert_eq!(
    e("((lambda ()
            (define x 5)
            (+ x ((lambda ()
                    (define x 11)
                    x)))))"), Value::Int(16));
}
