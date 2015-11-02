extern crate ares;

#[macro_use]
mod util;


#[test]
fn global_define() {
    eval_ok!("(define x 5)
       (+ x 1)", 6);
}

#[test]
fn lambda_define() {
    /*
    eval_ok!("((lambda ()
            (define x 5)
            x))", 5);*/

    eval_ok!("((lambda ()
            (define x 5)
            (+ x ((lambda ()
                    (define x 11)
                    x)))))", 16);
}
