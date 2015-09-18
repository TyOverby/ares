use std::rc::Rc;

use ::{Value, AresResult, rc_to_usize, write_usize};

pub use super::environment::{Env, Environment};

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    pub function: FfType
}

#[derive(Clone)]
pub enum FfType{
    FreeFn(Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>),
    //ContextFn(Rc<Fn(&mut T, &mut Iterator<Item=Value>) -> Value>),
    UnEvalFn(Rc<Fn(&mut Iterator<Item=&Value>, &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>)
}

impl ForeignFunction {
    pub fn new_free_function(
        name: String,
        function: Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>)
        -> ForeignFunction
    {
        ForeignFunction {
            name: name,
            function: FfType::FreeFn(function)
        }
    }

    pub fn new_uneval_function(
        name: String,
        function: Rc<Fn(&mut Iterator<Item=&Value>, &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>) -> ForeignFunction
    {
        ForeignFunction {
            name: name,
            function: FfType::UnEvalFn(function)
        }
    }

    fn to_usize(&self) -> usize {
        match &self.function {
            &FfType::FreeFn(ref rc) => {
                rc_to_usize(rc)
            }
            &FfType::UnEvalFn(ref rc) => {
                rc_to_usize(rc)
            }
        }
    }
}

impl ::std::fmt::Debug for ForeignFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str(&self.name)
    }
}

impl PartialEq for ForeignFunction {
    fn eq(&self, other: &ForeignFunction) -> bool {
        self.name == other.name &&
        self.to_usize() == other.to_usize()
    }
}

impl Eq for ForeignFunction {}

impl ::std::hash::Hash for ForeignFunction {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(self.to_usize(), state);
    }
}

