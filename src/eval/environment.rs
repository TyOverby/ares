use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

use ::Value;

pub type Env<S> = Rc<RefCell<Environment<S>>>;
pub struct Environment<S> {
    parent: Option<Env<S>>,
    bindings: HashMap<String, Value<S>>
}

impl <S> Environment<S>  {
    pub fn new() -> Environment<S> {
        Environment {
            parent: None,
            bindings: HashMap::new()
        }
    }

    pub fn new_with_data(env: Env<S>, bindings: HashMap<String, Value<S>>) -> Env<S> {
        Rc::new(RefCell::new(Environment {
            parent: Some(env),
            bindings: bindings
        }))
    }

    pub fn is_defined_at_this_level(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.with_value(name, |_| ()).is_some()
    }

    pub fn get(&self, name: &str) -> Option<Value<S>> {
        if self.bindings.contains_key(name) {
            Some(self.bindings[name].clone())
        } else if let Some(ref p) = self.parent {
            let lock = p.borrow();
            lock.get(name).clone()
        } else {
            None
        }
    }

    pub fn with_value<F, R>(&self, name: &str, function: F) -> Option<R>
    where F: FnOnce(&Value<S>) -> R
    {
        if self.bindings.contains_key(name) {
            Some(function(&self.bindings[name]))
        } else if let Some(ref p) = self.parent {
            let lock = p.borrow();
            lock.with_value(name, function)
        } else {
            None
        }
    }

    pub fn with_value_mut<F, R>(&mut self, name: &str, function: F) -> Option<R>
    where F: FnOnce(&mut Value<S>) -> R
    {
        if self.bindings.contains_key(name) {
            Some(function(self.bindings.get_mut(name).unwrap()))
        } else if let Some(ref p) = self.parent {
            let mut lock = p.borrow_mut();
            lock.with_value_mut(name, function)
        } else {
            None
        }
    }

    pub fn insert_here<N: Into<String>>(&mut self, name: N, values: Value<S>) -> Option<Value<S>> {
        self.bindings.insert(name.into(), values)
    }
}

