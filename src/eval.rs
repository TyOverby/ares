use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::{Value, AresError, AresResult};

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    function: FfType
}

#[derive(Clone)]
enum FfType{
    FreeFn(Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>),
    //ContextFn(Rc<Fn(&mut T, &mut Iterator<Item=Value>) -> Value>),
    UnEvalFn(Rc<Fn(&mut Iterator<Item=&Value>, &Env, fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>)
}

#[derive(Clone)]
pub struct Procedure {
    pub bodies: Rc<Vec<Value>>,
    param_names: Vec<String>, // TODO: allow this to also be a single identifier for varargs
    environment: Env
}

pub type Env = Rc<RefCell<Environment>>;
pub struct Environment {
    parent: Option<Env>,
    bindings: HashMap<String, Value>
}

impl ForeignFunction {
    fn new_free_function(name: String, function: Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>>) -> ForeignFunction {
        ForeignFunction {
            name: name,
            function: FfType::FreeFn(function)
        }
    }

    fn new_uneval_function(
        name: String,
        function: Rc<Fn(&mut Iterator<Item=&Value>, &Env, fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>>) -> ForeignFunction
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

fn rc_to_usize<T: ?Sized>(rc: &Rc<T>) -> usize {
    use std::mem::transmute;
    unsafe {transmute(&*rc)}
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

fn write_usize<H: ::std::hash::Hasher>(v: usize, hasher: &mut H) {
    use std::mem::transmute;
    unsafe {
        if cfg!(target_pointer_width = "32") {
            hasher.write(&transmute::<_, [u8; 4]>((v as u32)))
        } else {
            hasher.write(&transmute::<_, [u8; 8]>((v as u64)))
        }
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
    pub fn new(bodies: Rc<Vec<Value>>, param_names: Vec<String>, env: Env) -> Procedure {
        Procedure {
            bodies: bodies,
            param_names: param_names,
            environment: env
        }
    }

    fn gen_env<I: Iterator<Item=Value>>(&self, values: I) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            parent: Some(self.environment.clone()),
            bindings: self.param_names.iter().cloned().zip(values).collect()
        }))
    }
}


impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            bindings: HashMap::new()
        }
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

    pub fn insert(&mut self, name: String, value: Value) -> Option<Value> {
        self.bindings.insert(name, value)
    }

    pub fn set_function<F>(&mut self, name: &str, f: F)
    where F: Fn(&mut Iterator<Item=Value>) -> AresResult<Value> + 'static
    {
        let boxed: Rc<Fn(&mut Iterator<Item=Value>) -> AresResult<Value>> = Rc::new(f);
        self.bindings.insert(
            name.to_string(),
            Value::ForeignFn(ForeignFunction::new_free_function(name.to_string(), boxed)));
    }

    pub fn set_uneval_function<F>(&mut self, name: &str, f: F)
    where F: Fn(&mut Iterator<Item=&Value>, &Env, fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> + 'static
    {
        let boxed: Rc<Fn(&mut Iterator<Item=&Value>, &Env, fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value>> = Rc::new(f);
        self.bindings.insert(
            name.to_string(),
            Value::ForeignFn(ForeignFunction::new_uneval_function(name.to_string(), boxed)));
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
                None => panic!("Variable {} not found", ident)
            }
        }

        &Value::List(ref l) => {
            let mut items = l.iter();
            let head = items.next().unwrap();
            match try!(eval(head, env)) {
                Value::Lambda(procedure) => {
                    let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                    let evald = try!(evald);
                    let new_env = procedure.gen_env(evald.into_iter());
                    let mut last = None;
                    for body in &*procedure.bodies {
                        last = Some(try!(eval(body, &new_env)));
                    }
                    Ok(last.unwrap())
                }
                Value::ForeignFn(ff) => {
                    match ff.function {
                        FfType::FreeFn(ff) => {
                            let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                            let evald = try!(evald);
                            (ff)(&mut evald.into_iter())
                        }
                        FfType::UnEvalFn(uef) => (uef)(&mut items, env, eval)
                    }
                }
                x => panic!("{:?} is not executable", x)
            }
        }
    }
}

