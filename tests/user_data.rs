#![allow(dead_code)]

extern crate ares;

#[macro_use]
mod util;

use ares::{Context, Value, user_fn, AresError};

#[test]
fn user_data() {
    let mut ctx = Context::new();
    ctx.set_fn("ret-tuple",
               user_fn(
                   "ret-tuple",
                   |_, _| Ok(Value::user_data((1u32, "hi")))));

    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    let tup = ctx.eval_str("(ret-tuple)").unwrap();
    match tup {
        Value::UserData(data) => {
            assert!(data.is::<(u32, &'static str)>());
        }
        _ => assert!(false, "was not user data")
    }
}

#[test]
fn user_err() {
    let mut ctx = Context::new();
    ctx.set_fn("ret-tuple",
               user_fn(
                   "ret-tuple",
                   |_, _| Err(AresError::user_error((1u32, "hi")))));

    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    let tup = ctx.eval_str("(ret-tuple)").unwrap_err();
    match tup {
        AresError::UserError(data) => {
            assert!(data.is::<(u32, &'static str)>());
        }
        _ => assert!(false, "was not user data")
    }
}
