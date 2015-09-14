use std::rc::Rc;

mod tokenizer;
mod eval;
pub mod stdlib;
mod error;

pub use tokenizer::parse;
pub use eval::{Procedure, Environment, eval, ForeignFunction};
pub use error::{AresError, AresResult};

#[derive(Debug, Clone)]
pub enum Value {
    List(Rc<Vec<Value>>),
    String(Rc<String>),
    Float(f64),
    Int(i64),
    Bool(bool),

    Ident(Rc<String>),
    ForeignFn(ForeignFunction),
    Lambda(Procedure)
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use ::Value::*;
        fn rc_to_usize<T: ?Sized>(rc: &Rc<T>) -> usize {
            use std::mem::transmute;
            unsafe {transmute(&*rc)}
        }

        match (self, other) {
            (&List(ref rc1), &List(ref rc2)) =>
                rc_to_usize(rc1) == rc_to_usize(rc2),
            (&String(ref rc1), &String(ref rc2)) =>
                rc_to_usize(rc1) == rc_to_usize(rc2) || rc1 == rc2,
            (&Float(f1), &Float(f2)) => f1 == f2,
            (&Int(i1), &Int(i2)) => i1 == i2,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Ident(ref id1), &Ident(ref id2)) =>
                rc_to_usize(id1) == rc_to_usize(id2) || id1 == id2,
            (&ForeignFn(ref ff1), &ForeignFn(ref ff2)) => ff1 == ff2,
            (&Lambda(ref l1), &Lambda(ref l2)) => l1 == l2,
            _ => false
        }
    }
}

impl Eq for Value {}

// TODO: Ty, work on implementing Hash for this!


