extern crate ares;

use ::ares::*;

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

pub fn e(program: &str) -> AresResult<Value> {
    let mut ctx = Context::new();
    let mut ctx = ctx.load();
    ctx.eval_str(program)
}
