extern crate ares;

use ::ares::*;

use std::rc::Rc;
use std::cell::RefCell;

#[macro_export]
macro_rules! eval_ok {
    ($prog: expr, $v: expr) => {
        assert_eq!(util::e($prog).unwrap(), $v.into());
    }
}

macro_rules! eval_err {
    ($prog: expr, $p: pat) => {
        match util::e($prog) {
            Ok(v) => {
                panic!("eval_err! had a value: {:?}", v);
            }
            Err($p) => { assert!(true) },
            Err(v) => {
                panic!("eval_err! didn't match: {:?} was not {:?}", v, stringify!($p))
            }
        }
    }
}

fn basic_environment() -> Rc<RefCell<Environment>> {
    let mut env = Environment::new();
    stdlib::load_all(&mut env);
    Rc::new(RefCell::new(env))
}

pub fn e(program: &str) -> AresResult<Value> {
    let trees = parse(program).unwrap();
    let mut env = basic_environment();
    let mut last = None;
    for tree in trees {
        last = Some(try!(eval(&tree, &mut env)))
    }
    Ok(last.expect("no program found"))
}
