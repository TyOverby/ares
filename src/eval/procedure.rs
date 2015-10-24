use std::rc::Rc;
use std::collections::HashMap;

use ::{Value, AresError, AresResult, rc_to_usize, write_usize};

pub use super::environment::{Env, Environment};
use ::intern::Symbol;

#[derive(Clone, Eq, PartialEq)]
pub struct ParamBinding {
    pub params: Vec<Symbol>,
    pub rest: Option<Symbol>
}

#[derive(Clone)]
pub struct Procedure {
    pub name: Option<String>,
    pub bodies: Rc<Vec<Value>>,
    param_names: ParamBinding,
    environment: Env
}

impl Procedure {
    pub fn new(name: Option<String>, bodies: Rc<Vec<Value>>, param_names: ParamBinding, env: Env) -> Procedure {
        Procedure {
            name: name,
            bodies: bodies,
            param_names: param_names,
            environment: env
        }
    }

    pub fn gen_env<I: Iterator<Item=Value>>(&self, values: I) -> AresResult<Env> {
        let params_expected = self.param_names.params.len();
        let has_rest = self.param_names.rest.is_some();
        let params: Vec<_> = values.collect();
        if params.len() < params_expected {
            return Err(AresError::UnexpectedArity {
                found: params.len() as u16,
                expected: format!("{} {}", if has_rest { "at least" } else { "exactly " }, params.len())
            })
        }
        let named : Vec<_> = params[..params_expected].into();
        let mut bindings : HashMap<Symbol, Value> = self.param_names.params.iter().cloned().zip(named).collect();
        match self.param_names.rest {
            Some(rest_sym) => {
                let vec: Vec<_> = params[params_expected..].into();
                let list: Value = vec.into();
                bindings.insert(rest_sym.clone(), list);
            },
            None => ()
        };
        Ok(Environment::new_with_data(
            self.environment.clone(),
            bindings
                )
           )
    }
}

impl PartialEq for Procedure {
    fn eq(&self, other: &Procedure) -> bool {
        rc_to_usize(&self.bodies) == rc_to_usize(&other.bodies) &&
        rc_to_usize(&self.environment) == rc_to_usize(&other.environment)
    }
}

impl Eq for Procedure {}

impl ::std::fmt::Debug for Procedure {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str("<lambda>")
    }
}

impl ::std::hash::Hash for Procedure {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.bodies), state);
        write_usize(rc_to_usize(&self.environment), state);
    }
}
