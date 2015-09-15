use std::rc::Rc;
use std::cell::RefCell;

use super::{Value, AresError, AresResult, rc_to_usize, write_usize};

pub use self::environment::{Env, Environment};

mod environment;

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    function: FfType
}

#[derive(Clone)]
pub enum FfType{
    FreeFn(Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>),
    //ContextFn(Rc<Fn(&mut T, &mut Iterator<Item=Value>) -> Value>),
    UnEvalFn(Rc<Fn(&mut Iterator<Item=&Value>, &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>)
}

#[derive(Clone)]
pub struct Procedure {
    pub name: Option<String>,
    pub bodies: Rc<Vec<Value>>,
    param_names: Vec<String>, // TODO: allow this to also be a single identifier for varargs
    environment: Env
}

impl ForeignFunction {
    pub fn new_free_function(name: String, function: Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>) -> ForeignFunction {
        ForeignFunction {
            name: name,
            function: FfType::FreeFn(function)
        }
    }

    fn new_uneval_function(
        name: String,
        function: Rc<Fn(&mut Iterator<Item=&Value>, &Env, &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>) -> ForeignFunction
    {
        ForeignFunction {
            name: name,
            function: FfType::UnEvalFn(function)
        }
    }

    fn to_usize(&self) -> usize {
        match &self.function {
            &FfType::FreeFn(ref rc) => {
                rc_to_usize(rc)
            }
            &FfType::UnEvalFn(ref rc) => {
                rc_to_usize(rc)
            }
        }
    }
}

impl ::std::fmt::Debug for ForeignFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str(&self.name)
    }
}

impl PartialEq for ForeignFunction {
    fn eq(&self, other: &ForeignFunction) -> bool {
        self.name == other.name &&
        self.to_usize() == other.to_usize()
    }
}

impl Eq for ForeignFunction {}

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

impl ::std::hash::Hash for ForeignFunction {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(self.to_usize(), state);
    }
}

impl ::std::hash::Hash for Procedure {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.bodies), state);
        write_usize(rc_to_usize(&self.environment), state);
    }
}



impl Procedure {
    pub fn new(name: Option<String>, bodies: Rc<Vec<Value>>, param_names: Vec<String>, env: Env) -> Procedure {
        Procedure {
            name: name,
            bodies: bodies,
            param_names: param_names,
            environment: env
        }
    }

    pub fn gen_env<I: Iterator<Item=Value>>(&self, values: I) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(
                    Environment::new_with_data(
                        self.environment.clone(),
                        self.param_names.iter().cloned().zip(values).collect())))
    }
}


pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> AresResult<Value> {
    match value {
        &ref v@Value::String(_) => Ok(v.clone()),
        &ref v@Value::Int(_) => Ok(v.clone()),
        &ref v@Value::Float(_) => Ok(v.clone()),
        &ref v@Value::Bool(_) => Ok(v.clone()),

        &ref v@Value::ForeignFn(_) => Ok(v.clone()),
        &ref v@Value::Lambda(_) => Ok(v.clone()),

        &Value::Ident(ref ident) => {
            match env.borrow().get(&ident) {
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName((**ident).clone()))
            }
        }

        &Value::List(ref l) => {
            let mut items = l.iter();
            let head = match items.next() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList)
            };

            match try!(eval(head, env)) {
                Value::Lambda(procedure) => {
                    let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                    let evald = try!(evald);
                    let new_env = procedure.gen_env(evald.into_iter());
                    let mut last = None;
                    for body in &*procedure.bodies {
                        last = Some(try!(eval(body, &new_env)));
                    }
                    last.ok_or(AresError::NoLambdaBody)
                }
                Value::ForeignFn(ff) => {
                    match ff.function {
                        FfType::FreeFn(ff) => {
                            let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                            let evald = try!(evald);
                            (ff)(&mut evald.into_iter())
                        }
                        FfType::UnEvalFn(uef) => (uef)(&mut items, env, &|v, e| eval(v, e))
                    }
                }
                x => Err(AresError::UnexecutableValue(x))
            }
        }
    }
}

