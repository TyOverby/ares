use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::{Value};

#[derive(Clone)]
pub struct ForeignFunction {
    pub name: String,
    function: FfType
}

#[derive(Clone)]
enum FfType{
    FreeFn(Rc<Fn(&mut Iterator<Item=Value>) -> Value>),
    //ContextFn(Rc<Fn(&mut T, &mut Iterator<Item=Value>) -> Value>),
    UnEvalFn(Rc<Fn(&mut Iterator<Item=&Value>, &FnMut(&Value) -> Value) -> Value>)
}

#[derive(Clone)]
pub struct Procedure {
    pub body: Rc<Value>,
    param_names: Vec<String>, // TODO: allow this to also be a single identifier for varargs
    environment: Rc<RefCell<Environment>>
}

pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<String, Value>
}

impl ForeignFunction {
    fn new_free_function(name: String, function: Rc<Fn(&mut Iterator<Item=Value>) -> Value>) -> ForeignFunction {
        ForeignFunction {
            name: name,
            function: FfType::FreeFn(function)
        }
    }

    fn new_uneval_function(
        name: String,
        function: Rc<Fn(&mut Iterator<Item=&Value>, &FnMut(&Value) -> Value) -> Value>) -> ForeignFunction
    {
        ForeignFunction {
            name: name,
            function: FfType::UnEvalFn(function)
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
        use std::mem::transmute;
        let a: *mut () = unsafe{ transmute(&self.function) };
        let b: *mut () = unsafe{ transmute(&other.function) };
        a == b
    }
}


impl PartialEq for Procedure {
    fn eq(&self, other: &Procedure) -> bool {
        use std::mem::transmute;
        let a: *mut () = unsafe{ transmute(&self.body) };
        let b: *mut () = unsafe{ transmute(&other.body) };

        let c: *mut () = unsafe{ transmute(&self.environment) };
        let d: *mut () = unsafe{ transmute(&other.environment) };

        a == b && c == d
    }
}

impl ::std::fmt::Debug for Procedure {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str("<lambda>")
    }
}

impl Procedure {
    fn new(body: Rc<Value>, param_names: Vec<String>, env: Rc<RefCell<Environment>>) -> Procedure {
        Procedure {
            body: body,
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

    pub fn set_function<F>(&mut self, name: &str, f: F)
    where F: Fn(&mut Iterator<Item=Value>) -> Value + 'static
    {
        let boxed: Rc<Fn(&mut Iterator<Item=Value>) -> Value> = Rc::new(f);
        self.bindings.insert(
            name.to_string(),
            Value::ForeignFn(ForeignFunction::new_free_function(name.to_string(), boxed)));
    }

    pub fn set_uneval_function<F>(&mut self, name: &str, f: F)
    where F: Fn(&mut Iterator<Item=&Value>, &FnMut(&Value) -> Value) -> Value + 'static
    {
        let boxed: Rc<Fn(&mut Iterator<Item=&Value>, &FnMut(&Value) -> Value) -> Value> = Rc::new(f);
        self.bindings.insert(
            name.to_string(),
            Value::ForeignFn(ForeignFunction::new_uneval_function(name.to_string(), boxed)));
    }
}

pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> Value {
    match value {
        &ref v@Value::String(_) => v.clone(),
        &ref v@Value::Int(_) => v.clone(),
        &ref v@Value::Float(_) => v.clone(),
        &ref v@Value::Bool(_) => v.clone(),

        &ref v@Value::ForeignFn(_) => v.clone(),
        &ref v@Value::Lambda(_) => v.clone(),

        &Value::Ident(ref ident) => {
            match env.borrow().get(&ident) {
                Some(env) => env,
                None => panic!("Variable {} not found", ident)
            }
        }

        &Value::List(ref l) => {
            let mut items = l.iter();
            match items.next().unwrap() {
                &Value::Ident(ref v) if &**v == "quote" => {
                    items.next().unwrap().clone()
                }
                &Value::Ident(ref v) if &**v == "lambda" => {
                    let names = items.next().unwrap();
                    let body  = items.next().unwrap();

                    let param_names = match &*names {
                        &Value::List(ref v) => {
                            let r: Vec<String> = v.iter().map(|n| {
                                if let &Value::Ident(ref s) = n {
                                    (&**s).clone()
                                } else {
                                    panic!("non ident param name");
                                }
                            }).collect();
                            r
                        }
                        _ => panic!("no param names list found for lambda")
                    };

                    Value::Lambda(Procedure::new(Rc::new(body.clone()), param_names, env.clone()))
                }
                &Value::Ident(ref v) if &**v == "define" => {
                    let name: String = if let &Value::String(ref s) = items.next().unwrap() {
                        (&**s).clone()
                    } else {
                        panic!("define with no name");
                    };
                    let value = items.next().unwrap();
                    let result = eval(value, env);
                    env.borrow_mut().bindings.insert(name, result.clone());
                    result
                }
                &Value::Ident(ref v) if &**v == "if" => {
                    let true_cond = items.next().unwrap();
                    let false_cond = items.next().unwrap();
                    match eval(items.next().unwrap(), env) {
                        Value::Bool(true) => eval(true_cond, env),
                        Value::Bool(false) => eval(false_cond, env),
                        _ => panic!("boolean expected in 'if'")
                    }
                }
                other => {
                    match eval(other, env) {
                        Value::Lambda(procedure) => {
                            let new_env = procedure.gen_env(items.map(|v| eval(v, env)));
                            eval(&procedure.body, &new_env)
                        }
                        Value::ForeignFn(ff) => {
                            match ff.function {
                                FfType::FreeFn(ff) => (ff)(&mut items.map(|v| eval(v, env))),
                                FfType::UnEvalFn(uef) => (uef)(&mut items, &|v| eval(v, env))
                            }
                        }
                        x => panic!("{:?} is not executable", x)
                    }
                }
            }
        }
    }
}

