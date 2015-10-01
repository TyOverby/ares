#![allow(mutable_transmutes)]

use std::rc::Rc;

pub mod tokenizer;
mod eval;
pub mod stdlib;
mod error;
//pub mod util;

pub use tokenizer::parse;
pub use eval::{
    LoadedContext,
    Procedure,
    eval,
    apply,
    ForeignFunction,
    Env,
    Environment,
    ParamBinding,
    free_fn,
    ast_fn,
    Context
};
pub use error::{AresError, AresResult};

macro_rules! gen_from {
    ($inx: ty, $out: path) => {
        gen_from!($inx, $out, |i| i);
    };
    ($inx: ty, $out: path, $tr: expr) => {
        impl <S> From<$inx> for Value<S> {
            fn from(i: $inx) -> Value<S> {
                $out($tr(i))
            }
        }
    }
}

#[derive(Debug)]
pub enum Value<S> {
    List(Rc<Vec<Value<S>>>),
    String(Rc<String>),
    Float(f64),
    Int(i64),
    Bool(bool),

    Ident(Rc<String>),
    ForeignFn(ForeignFunction<S>),
    Lambda(Procedure<S>)
}

impl <S> Value<S> {
    pub fn new_string<N: Into<String>>(s: N) -> Value<S> {
        Value::String(Rc::new(s.into()))
    }
    pub fn new_ident<N: Into<String>>(s: N) -> Value<S> {
        Value::Ident(Rc::new(s.into()))
    }
    pub fn new_list(v: Vec<Value<S>>) -> Value<S> {
        Value::List(Rc::new(v))
    }
}

impl <S> Clone for Value<S> {
    fn clone(&self) -> Value<S> {
        match self {
            &Value::List(ref inner) => Value::List(inner.clone()),
            &Value::String(ref inner) => Value::String(inner.clone()),
            &Value::Float(f) => Value::Float(f),
            &Value::Int(i) => Value::Int(i),
            &Value::Bool(b) => Value::Bool(b),
            &Value::Ident(ref inner) => Value::Ident(inner.clone()),
            &Value::ForeignFn(ref inner) => Value::ForeignFn((*inner).clone()),
            &Value::Lambda(ref inner) => Value::Lambda((*inner).clone())
        }
    }
}

gen_from!(u8, Value::Int, |a| a as i64);
gen_from!(i8, Value::Int, |a| a as i64);
gen_from!(u16, Value::Int, |a| a as i64);
gen_from!(i16, Value::Int, |a| a as i64);
gen_from!(u32, Value::Int, |a| a as i64);
gen_from!(i32, Value::Int, |a| a as i64);
gen_from!(u64, Value::Int, |a| a as i64);
gen_from!(i64, Value::Int);

gen_from!(f32, Value::Float, |a| a as f64);
gen_from!(f64, Value::Float);

gen_from!(bool, Value::Bool);

gen_from!(String, Value::String, Rc::new);

impl <S, T: Into<Value<S>>> From<Vec<T>> for Value<S> {
    fn from(x: Vec<T>) -> Value<S> {
        Value::List(Rc::new(x.into_iter().map(|a| a.into()).collect()))
    }
}

impl <'a, S> From<&'a str> for Value<S> {
    fn from(x: &'a str) -> Value<S> {
        let s: String = x.into();
        let v: Value<S> = s.into();
        v
    }
}

impl <S> PartialEq for Value<S> {
    fn eq(&self, other: &Value<S>) -> bool {
        use ::Value::*;

        match (self, other) {
            (&List(ref rc1), &List(ref rc2)) => rc1 == rc2,
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

impl <S> Eq for Value<S> {}

impl <S> std::hash::Hash for Value<S> {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        use std::mem::transmute;
        match self {
            &Value::List(ref rc) => rc.hash(state),
            &Value::String(ref rc) => rc.hash(state),
            &Value::Float(f) => unsafe { state.write(&transmute::<_, [u8; 8]>(f)) },
            &Value::Int(i) => unsafe { state.write(&transmute::<_, [u8; 8]>(i)) },
            &Value::Bool(b) => state.write(&[if b {1} else {0}]),
            &Value::Ident(ref rc) => rc.hash(state),
            &Value::ForeignFn(ref ff) => ff.hash(state),
            &Value::Lambda(ref p) => p.hash(state),
        }
    }
}

fn write_usize<H: ::std::hash::Hasher>(v: usize, hasher: &mut H) {
    use std::mem::transmute;
    unsafe {
        if cfg!(target_pointer_width = "32") {
            hasher.write(&transmute::<_, [u8; 4]>((v as u32)))
        } else {
            hasher.write(&transmute::<_, [u8; 8]>((v as u64)))
        }
    }
}

fn rc_to_usize<T: ?Sized>(rc: &Rc<T>) -> usize {
    use std::mem::transmute;
    unsafe {transmute(&*rc)}
}
