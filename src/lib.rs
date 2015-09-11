use std::rc::Rc;

mod tokenizer;
mod eval;
pub mod stdlib;

pub use tokenizer::parse;
pub use eval::{Procedure, Environment, eval, ForeignFunction};


#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
pub enum Error {
    UnexpectedType{expected: String, found: String, at: u32, in_: String}
}


