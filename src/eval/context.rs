use std::rc::Rc;
use std::cell::RefCell;
//use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::{Env, eval, apply};
use ::{Value, AresResult, AresError, parse, stdlib, Environment};

pub struct Context {
    env: Env,
    //_state: PhantomData<T>,
}

pub struct LoadedContext<'a> {
    ctx: &'a mut Context,
    //state: &'a mut T,
}

impl Context {
    pub fn new() -> Context {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        stdlib::load_all(&mut env);
        Context {
            env: env,
            //_state: PhantomData,
        }
    }

    pub fn new_empty() -> Context {
        Context {
            env: Rc::new(RefCell::new(Environment::new())),
            //_state: PhantomData,
        }
    }

    pub fn load<'a>(&'a mut self) -> LoadedContext<'a> {
        LoadedContext {
            ctx: self,
        }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }
}

impl <'a> LoadedContext<'a> {
    pub fn with_other_env<F, R>(&mut self, env: &mut Env, f: F) -> R
    where F: FnOnce(&mut LoadedContext<'a>) -> R {
        use std::mem::swap;
        swap(&mut self.ctx.env, env);
        let r = f(self);
        swap(&mut self.ctx.env, env);
        r
    }

    pub fn eval(&mut self, value: &Value) -> AresResult<Value> {
        eval(value, self)
    }

    pub fn eval_str(&mut self, program: &str) -> AresResult<Value> {
        let trees = try!(parse(program));
        let mut last = None;
        for tree in trees {
            last = Some(try!(self.eval(&tree)))
        }
        match last {
            Some(v) => Ok(v),
            None => Err(AresError::NoProgram)
        }
    }

    pub fn call(&mut self, func: &Value, args: &[Value]) -> AresResult<Value> {
        apply(func, &args[..], self)
    }

    pub fn call_named<S: AsRef<str>>(&mut self, global_fn: S, args: &[Value]) -> AresResult<Value> {
        let func = self.env.borrow().get(global_fn.as_ref());
        match func {
            Some(v) => self.call(&v, args),
            None => Err(AresError::UndefinedName(global_fn.as_ref().into()))
        }
    }
}

impl <'a> Deref for LoadedContext<'a> {
    type Target = Context;
    fn deref(&self) -> &Context {
        &self.ctx
    }
}

impl <'a> DerefMut for LoadedContext<'a> {
    fn deref_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}
