use std::rc::Rc;

use ::{Value, AresResult, rc_to_usize, write_usize, LoadedContext, eval};

pub use super::environment::{Env, Environment};

pub struct ForeignFunction<S> {
    pub name: String,
    pub function: Rc<Fn(&[Value<S>], &mut LoadedContext<S>) -> AresResult<Value<S>, S>>
}

impl <S> Clone for ForeignFunction<S> {
    fn clone(&self) -> ForeignFunction<S> {
        ForeignFunction {
            name: self.name.clone(),
            function: self.function.clone()
        }
    }
}

pub fn free_fn<N, F, S: 'static>(name: N, func: F) -> Value<S>
where N: Into<String>,
      F: Fn(&[Value<S>]) -> AresResult<Value<S>, S> + 'static
{
    let closure = move |values: &[Value<S>], ctx: &mut LoadedContext<S> | {
        let evaluated: Result<Vec<_>, _> = values.iter().map(|v| eval(v, ctx)).collect();
        let evaluated = try!(evaluated);
        func(&evaluated[..])
    };

    let boxed = Rc::new(closure);
    Value::ForeignFn(ForeignFunction {
        name: name.into(),
        function: boxed
    })
}


pub fn ast_fn<N, F, S: 'static>(name: N, func: F) -> Value<S>
where N: Into<String>,
      F: Fn(&[Value<S>], &mut LoadedContext<S>) -> AresResult<Value<S>, S> + 'static
{
    let boxed = Rc::new(func);
    Value::ForeignFn(ForeignFunction {
        name: name.into(),
        function: boxed
    })
}


impl <S> ::std::fmt::Debug for ForeignFunction<S> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str(&self.name)
    }
}

impl <S> PartialEq for ForeignFunction<S> {
    fn eq(&self, other: &ForeignFunction<S>) -> bool {
        self.name == other.name &&
        rc_to_usize(&self.function) == rc_to_usize(&other.function)
    }
}

impl <S> Eq for ForeignFunction<S> {}

impl <S> ::std::hash::Hash for ForeignFunction<S> {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.function), state);
    }
}

