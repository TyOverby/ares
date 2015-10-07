#![allow(unused)]

extern crate ares;
use ::ares::*;

macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => ({
        let mut m = HashMap::new();
        $(m.insert($k, $v));*;
        m
    })
}

#[macro_export]
macro_rules! eval_ok {
    ($prog: expr) => {
        assert!(util::e($prog).is_ok());
    };
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

macro_rules! s {
    ($s:expr) => { ares::Value::symbol($s) }
}

macro_rules! v {
    ($($s:expr),*) => { ares::Value::list(vec![$($s),*]) }
}

pub fn e(program: &str) -> AresResult<Value> {
    let mut ctx = Context::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);
    ctx.eval_str(program)
}
