use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

mod parse;
mod eval;
pub mod stdlib;
mod error;
pub mod util;
pub mod intern;

pub use parse::parse;
pub use eval::{
    user_fn,
    free_fn,
    ast_fn,
    Procedure,
    ForeignFunction,
    Env,
    Environment,
    ParamBinding,
    Context,
    LoadedContext,
    State,
};
pub use error::{AresError, AresResult};

macro_rules! gen_from {
    ($inx: ty, $out: path) => {
        gen_from!($inx, $out, |i| i);
    };
    ($inx: ty, $out: path, $tr: expr) => {
        impl From<$inx> for Value {
            fn from(i: $inx) -> Value {
                $out($tr(i))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    List(Rc<Vec<Value>>),
    String(Rc<String>),
    Float(f64),
    Int(i64),
    Bool(bool),
    Map(Rc<HashMap<Value, Value>>),

    Symbol(intern::Symbol),
    ForeignFn(ForeignFunction<()>),
    Lambda(Procedure),

    UserData(Rc<Any>)
}

impl Value {
    pub fn string<S: Into<String>>(s: S) -> Value {
        Value::String(Rc::new(s.into()))
    }

    pub fn list(v: Vec<Value>) -> Value {
        Value::List(Rc::new(v))
    }

    pub fn user_data<T: Any>(t: T) -> Value {
        Value::UserData(Rc::new(t) as Rc<Any>)
    }
}

gen_from!(intern::Symbol, Value::Symbol, |a| a);
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

impl <T: Into<Value>> From<Vec<T>> for Value {
    fn from(x: Vec<T>) -> Value {
        Value::List(Rc::new(x.into_iter().map(|a| a.into()).collect()))
    }
}

impl <T: Into<Value> + std::hash::Hash + Eq> From<HashMap<T, T>> for Value {
    fn from(x: HashMap<T, T>) -> Value {
        Value::Map(Rc::new(x.into_iter().map(|(k, v)| (k.into(), v.into())).collect()))
    }
}

impl <'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let s: String = x.into();
        let v: Value = s.into();
        v
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use ::Value::*;

        match (self, other) {
            (&List(ref rc1), &List(ref rc2)) => rc1 == rc2,
            (&String(ref rc1), &String(ref rc2)) =>
                rc_to_usize(rc1) == rc_to_usize(rc2) || rc1 == rc2,
            (&Float(f1), &Float(f2)) => f1 == f2,
            (&Int(i1), &Int(i2)) => i1 == i2,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(ref id1), &Symbol(ref id2)) => id1 == id2,
            (&ForeignFn(ref ff1), &ForeignFn(ref ff2)) => ff1 == ff2,
            (&Lambda(ref l1), &Lambda(ref l2)) => l1 == l2,
            (&Map(ref m1), &Map(ref m2)) => m1 == m2,
            (&UserData(ref u1), &UserData(ref u2)) =>
                rc_to_usize(u1) == rc_to_usize(u2),
            _ => false
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        use std::mem::transmute;
        match self {
            &Value::List(ref rc) => rc.hash(state),
            &Value::String(ref rc) => rc.hash(state),
            &Value::Float(f) => unsafe { state.write(&transmute::<_, [u8; 8]>(f)) },
            &Value::Int(i) => unsafe { state.write(&transmute::<_, [u8; 8]>(i)) },
            &Value::Bool(b) => state.write(&[if b {1} else {0}]),
            &Value::Symbol(ref rc) => rc.hash(state),
            &Value::ForeignFn(ref ff) => ff.hash(state),
            &Value::Lambda(ref p) => p.hash(state),
            &Value::UserData(ref u) => write_usize(rc_to_usize(u), state),
            &Value::Map(_) => unimplemented!()  // hashmap not hashable.
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
