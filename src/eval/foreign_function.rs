use std::rc::Rc;

use ::{Value, AresResult, rc_to_usize, write_usize};

pub use super::environment::{Env, Environment};

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    pub function: Rc<Fn(&[Value], &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>
}

pub fn free_fn<S, F>(name: S, func: F) -> Value
where S: Into<String>,
      F: Fn(&[Value]) -> AresResult<Value> + 'static
{
    let closure = move |values: &[Value], env: &Env, eval: &Fn(&Value, &Env) -> AresResult<Value>| {
        let evaluated: Result<Vec<_>, _> = values.iter().map(|v| eval(v, env)).collect();
        let evaluated = try!(evaluated);
        func(&evaluated[..])
    };

    let boxed = Rc::new(closure);
    Value::ForeignFn(ForeignFunction {
        name: name.into(),
        function: boxed
    })
}


pub fn ast_fn<S, F>(name: S, func: F) -> Value
where S: Into<String>,
      F: Fn(&[Value], &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> + 'static
{
    let boxed = Rc::new(func);
    Value::ForeignFn(ForeignFunction {
        name: name.into(),
        function: boxed
    })
}


impl ::std::fmt::Debug for ForeignFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str(&self.name)
    }
}

impl PartialEq for ForeignFunction {
    fn eq(&self, other: &ForeignFunction) -> bool {
        self.name == other.name &&
        rc_to_usize(&self.function) == rc_to_usize(&other.function)
    }
}

impl Eq for ForeignFunction {}

impl ::std::hash::Hash for ForeignFunction {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.function), state);
    }
}

