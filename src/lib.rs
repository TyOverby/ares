use std::rc::Rc;

mod tokenizer;
mod eval;
pub mod stdlib;

pub use tokenizer::parse;
pub use eval::{Procedure, Environment, eval};

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    pub function: Rc<Fn(Vec<Value>) -> Value>
}

impl ForeignFunction {
    fn new(name: String, function: Rc<Fn(Vec<Value>) -> Value>) -> ForeignFunction {
        ForeignFunction {
            name: name,
            function: function
        }
    }
}

impl std::fmt::Debug for ForeignFunction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
        fmt.write_str(&self.name)
    }
}

impl PartialEq for ForeignFunction {
    fn eq(&self, other: &ForeignFunction) -> bool {
        use std::mem::transmute;
        let a: *mut () = unsafe{ transmute(&self.function) };
        let b: *mut () = unsafe{ transmute(&other.function) };
        a == b
    }
}

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


