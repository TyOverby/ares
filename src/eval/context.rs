use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::borrow::Cow;

use super::{Env, eval, apply};
use ::{Value, AresResult, AresError, parse, stdlib, Environment};

pub struct Context<S> {
    env: Env<S>,
    _state: PhantomData<S>,
}

pub struct LoadedContext<'a, S: 'a> {
    ctx: &'a mut Context<S>,
    override_env: Option<Env<S>>,
    state: &'a mut S,
}

impl <S: 'static> Context<S> {
    pub fn new() -> Context<S> {
        let env = Rc::new(RefCell::new(Environment::new()));
        let mut ctx = Context {
            env: env,
            _state: PhantomData,
        };

        stdlib::load_all(&mut ctx);

        ctx
    }

    pub fn new_empty() -> Context<S> {
        Context {
            env: Rc::new(RefCell::new(Environment::new())),
            _state: PhantomData,
        }
    }

    pub fn get<N: AsRef<str>>(&self, name: &N) -> Option<Value<S>> {
        self.env.borrow_mut().get(name.as_ref())
    }

    pub fn set<N: Into<String>>(&mut self, name: N, value: Value<S>) -> Option<Value<S>> {
        self.env.borrow_mut().insert_here(name, value)
    }

    pub fn load<'a>(&'a mut self, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext {
            ctx: self,
            state: state,
            override_env: None,
        }
    }

    pub fn env(&self) -> &Env<S> {
        &self.env
    }
}

impl <'a, S> LoadedContext<'a, S> {
    pub fn unload(self) { }

    pub fn env(&self) -> &Env<S> {
        self.override_env.as_ref().unwrap_or(&self.env)
    }

    pub fn eval(&mut self, value: &Value<S>) -> AresResult<Value<S>, S> {
        eval(value, self)
    }

    // TODO: hide from docs
    pub fn with_env<F, R>(&mut self, mut env: Env<S>, f: F) -> R
    where F: FnOnce(&mut LoadedContext<'a, S>) -> R
    {
        use std::mem::swap;
        swap(&mut self.env, &mut env);
        let r = f(self);
        swap(&mut self.env, &mut env);
        r
    }

    pub fn eval_str(&mut self, program: &str) -> AresResult<Value<S>, S> {
        let trees = try!(parse(program));
        let mut last = None;
        for tree in trees {
            last = Some(try!(eval(&tree, self)))
        }
        match last {
            Some(v) => Ok(v),
            None => Err(AresError::NoProgram)
        }
    }

    pub fn call(&mut self, func: &Value<S>, mut args: &[Value<S>]) -> AresResult<Value<S>, S> {
        apply(func, &args[..], self)
    }

    pub fn call_named<N: AsRef<str>>(&mut self, global_fn: N, args: &[Value<S>]) -> AresResult<Value<S>, S> {
        let func = self.env.borrow().get(global_fn.as_ref());
        match func {
            Some(v) => self.call(&v, args),
            None => Err(AresError::UndefinedName(global_fn.as_ref().into()))
        }
    }
}

impl <'a, S> Deref for LoadedContext<'a, S> {
    type Target = Context<S>;
    fn deref(&self) -> &Context<S> {
        &self.ctx
    }
}

impl <'a, S> DerefMut for LoadedContext<'a, S> {
    fn deref_mut(&mut self) -> &mut Context<S> {
        &mut self.ctx
    }
}
