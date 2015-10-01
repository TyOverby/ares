use std::rc::Rc;
use std::collections::HashMap;

use ::{Value, rc_to_usize, write_usize};

pub use super::environment::{Env, Environment};

#[derive(Clone, Eq, PartialEq)]
pub enum ParamBinding {
    SingleIdent(String),
    ParamList(Vec<String>)
}

pub struct Procedure<S> {
    pub name: Option<String>,
    pub bodies: Rc<Vec<Value<S>>>,
    param_names: ParamBinding, // TODO: allow this to also be a single identifier for varargs
    environment: Env<S>
}

impl <S> Procedure<S> {
    pub fn new(name: Option<String>, bodies: Rc<Vec<Value<S>>>, param_names: ParamBinding, env: Env<S>) -> Procedure<S> {
        Procedure {
            name: name,
            bodies: bodies,
            param_names: param_names,
            environment: env
        }
    }

    pub fn gen_env<I: Iterator<Item=Value<S>>>(&self, values: I) -> Env<S> {
        match &self.param_names {
            &ParamBinding::SingleIdent(ref s) => {
                let vec: Vec<_> = values.collect();
                let list: Value<S> = vec.into();
                let mut binding = HashMap::new();
                binding.insert(s.clone(), list);
                Environment::new_with_data(
                    self.environment.clone(),
                    binding)
            }
            &ParamBinding::ParamList(ref v) => {
                Environment::new_with_data(
                    self.environment.clone(),
                    v.iter().cloned().zip(values).collect())
            }
        }
    }
}

impl <S> Clone for Procedure<S> {
    fn clone(&self) -> Procedure<S> {
        Procedure {
            name: self.name.clone(),
            bodies: self.bodies.clone(),
            param_names: self.param_names.clone(),
            environment: self.environment.clone()
        }
    }
}

impl <S> PartialEq for Procedure<S> {
    fn eq(&self, other: &Procedure<S>) -> bool {
        rc_to_usize(&self.bodies) == rc_to_usize(&other.bodies) &&
        rc_to_usize(&self.environment) == rc_to_usize(&other.environment)
    }
}

impl <S> Eq for Procedure<S> {}

impl <S> ::std::fmt::Debug for Procedure<S>{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str("<lambda>")
    }
}

impl <S> ::std::hash::Hash for Procedure<S> {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.bodies), state);
        write_usize(rc_to_usize(&self.environment), state);
    }
}
