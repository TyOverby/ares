use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::any::Any;

use super::{Env, eval, apply};
use ::{Value, AresResult, AresError, parse, stdlib, Environment, ForeignFunction};
use ::intern::SymbolIntern;
use stdlib::core::macroexpand;

pub struct Context<S: State + ?Sized> {
    env: Env,
    interner: SymbolIntern,
    _state: PhantomData<S>,
}

pub struct LoadedContext<'a, S: State + ?Sized> {
    ctx: &'a mut Context<S>,
    state: &'a mut S,
}

pub trait State: Any {}

impl <T: Any> State for T {}

impl <S: State + ?Sized> Context<S> {
    pub fn new() -> Context<S> {
        let env = Rc::new(RefCell::new(Environment::new()));
        let mut ctx = Context {
            env: env,
            interner: SymbolIntern::new(),
            _state: PhantomData,
        };
        stdlib::load_all(&mut ctx);
        ctx
    }

    pub fn new_empty() -> Context<S> {
        Context {
            env: Rc::new(RefCell::new(Environment::new())),
            interner: SymbolIntern::new(),
            _state: PhantomData,
        }
    }

    pub fn with_debug(mut self) -> Context<S> {
        stdlib::load_debug(&mut self);
        self
    }

    pub fn format_value(&self, value: &Value) -> String {
        ::stdlib::types::to_string_helper(value, self.interner())
    }

    pub fn load<'a>(&'a mut self, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext {
            ctx: self,
            state: state
        }
    }

    pub fn get<N: ?Sized + AsRef<str>>(&self, name: &N) -> Option<Value> {
        if let Some(symbol) = self.interner.symbol_for_name(name) {
            self.env.borrow_mut().get(symbol)
        } else {
            None
        }
    }

    pub fn set<N: AsRef<str> + Into<String>>(&mut self, name: N, value: Value) -> Option<Value> {
        let ret = self.env.borrow_mut().insert_here(self.interner.intern(name), value);
        ret
    }

    pub fn set_fn<N: AsRef<str> + Into<String>>(&mut self, name: N, f: ForeignFunction<S>) -> Option<Value> {
        self.set(name, Value::ForeignFn(f.erase()))
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn env_mut(&mut self) -> &mut Env {
        &mut self.env
    }

    pub fn interner(&self) -> &SymbolIntern {
        &self.interner
    }

    pub fn interner_mut(&mut self) -> &mut SymbolIntern {
        &mut self.interner
    }
}

impl <'a, S: State + ?Sized> LoadedContext<'a, S> {
    pub fn with_other_env<F, R>(&mut self, env: &mut Env, f: F) -> R
    where F: FnOnce(&mut LoadedContext<'a, S>) -> R {
        use std::mem::swap;
        swap(&mut self.ctx.env, env);
        let r = f(self);
        swap(&mut self.ctx.env, env);
        r
    }

    pub fn state(&mut self) -> &mut S {
        self.state
    }

    pub fn eval(&mut self, value: &Value) -> AresResult<Value> {
        eval(value, self, false)
    }

    pub fn macroexpand(&mut self, value: Value) -> AresResult<Value> {
        macroexpand(&[value], self)
    }

    pub fn eval_str(&mut self, program: &str) -> AresResult<Value> {
        let trees = try!(parse(program, &mut self.interner));
        let mut last = None;
        for tree in trees {
            let tree = try!(self.macroexpand(tree));
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

    pub fn call_named<N: ?Sized + AsRef<str>>(&mut self, named_fn: &N, args: &[Value]) -> AresResult<Value> {
        let func = self.get(named_fn);
        match func {
            Some(v) => self.call(&v, args),
            None => Err(AresError::UndefinedName(named_fn.as_ref().into()))
        }
    }

    pub fn unload(self) {  }
}

impl <'a, S: State + ?Sized> Deref for LoadedContext<'a, S> {
    type Target = Context<S>;
    fn deref(&self) -> &Context<S> {
        &self.ctx
    }
}

impl <'a, S: State + ?Sized> DerefMut for LoadedContext<'a, S> {
    fn deref_mut(&mut self) -> &mut Context<S> {
        &mut self.ctx
    }
}
