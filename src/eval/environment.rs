use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

use ::Value;

pub type Env = Rc<RefCell<Environment>>;
pub struct Environment {
    parent: Option<Env>,
    bindings: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            bindings: HashMap::new()
        }
    }

    pub fn new_with_data(env: Env, bindings: HashMap<String, Value>) -> Env {
        Rc::new(RefCell::new(Environment {
            parent: Some(env),
            bindings: bindings
        }))
    }

    fn defined_helper(&self, values: &mut HashMap<String, (u32, Value)>, depth: u32) {
        for (k, v) in &self.bindings {
            if !values.contains_key(k) {
                values.insert(k.clone(), (depth, v.clone()));
            }
        }
        if let &Some(ref parent) = &self.parent {
            parent.borrow().defined_helper(values, depth + 1);
        }
    }

    pub fn all_defined(&self) -> HashMap<String, (u32, Value)> {
        let mut defined = HashMap::new();
        self.defined_helper(&mut defined, 0);
        defined
    }

    pub fn is_defined_at_this_level(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.with_value(name, |_| ()).is_some()
    }

    pub fn get(&self, name: &str) -> Option<Value> {
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
    where F: FnOnce(&Value) -> R
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
    where F: FnOnce(&mut Value) -> R
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

    pub fn insert_here<S: Into<String>>(&mut self, name: S, value: Value) -> Option<Value> {
        self.bindings.insert(name.into(), value)
    }
}

