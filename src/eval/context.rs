use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::{Env, eval, apply};
use ::{Value, AresResult, AresError, parse, stdlib, Environment};

pub struct Context<T> {
    env: Env,
    _state: PhantomData<T>,
}

pub struct LoadedContext<'a, 'b, T: 'a + 'b> {
    ctx: &'b mut Context<T>,
    state: &'a mut T,
}

impl <T> Context<T> {
    pub fn new() -> Context<T> {
        let env = Rc::new(RefCell::new(Environment::new()));
        stdlib::load_all(&env);
        Context {
            env: env,
            _state: PhantomData,
        }
    }

    pub fn new_empty() -> Context<T> {
        Context {
            env: Rc::new(RefCell::new(Environment::new())),
            _state: PhantomData,
        }
    }

    pub fn load<'a, 'b>(&'b mut self, state: &'a mut T) -> LoadedContext<'a, 'b, T> {
        LoadedContext {
            ctx: self,
            state: state
        }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }
}

impl <'a, 'b,  T> LoadedContext<'a, 'b, T> {
    pub fn eval(&mut self, value: &Value) -> AresResult<Value> {
        eval(value, self.env())
    }

    pub fn eval_str(&mut self, program: &str) -> AresResult<Value> {
        let trees = try!(parse(program));
        let mut last = None;
        for tree in trees {
            last = Some(try!(eval(&tree, self.env())))
        }
        match last {
            Some(v) => Ok(v),
            None => Err(AresError::NoProgram)
        }
    }

    pub fn call(&mut self, func: &Value, mut args: &[Value]) -> AresResult<Value> {
        apply(func, &args[..], self.env())
    }

    pub fn call_named<S: AsRef<str>>(&mut self, global_fn: S, args: &[Value]) -> AresResult<Value> {
        let func = self.env.borrow().get(global_fn.as_ref());
        match func {
            Some(v) => self.call(&v, args),
            None => Err(AresError::UndefinedName(global_fn.as_ref().into()))
        }
    }
}

impl <'a,'b, T > Deref for LoadedContext<'a, 'b, T > {
    type Target = Context<T>;
    fn deref(&self) -> &Context<T> {
        &self.ctx
    }
}

impl <'a, 'b, T> DerefMut for LoadedContext<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut Context<T> {
        &mut self.ctx
    }
}
