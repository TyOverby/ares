use std::rc::Rc;
use std::convert::AsRef;
use std::cell::RefCell;
use std::collections::HashMap;

use super::{Value, ForeignFunction};

#[derive(Clone)]
pub struct Procedure {
    pub body: Rc<Value>,
    param_names: Vec<String>, // TODO: allow this to also be a single identifier for varargs
    environment: Rc<RefCell<Environment>>
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

pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<String, Value>
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

    pub fn set_function<F: Fn(Vec<Value>) -> Value + 'static>(&mut self, name: &str, f: F) {
        let boxed: Rc<Fn(Vec<Value>) -> Value> = Rc::new(f);
        self.bindings.insert(name.to_string(), Value::ForeignFn(ForeignFunction::new(name.to_string(), boxed)));
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
                            items.map(|n| {
                                if let &Value::Ident(ref s) = n {
                                    (&**s).clone()
                                } else {
                                    panic!("non ident param name");
                                }
                            }).collect()
                        }
                        _ => panic!("no param names list found for lambda")
                    };

                    Value::Lambda(Procedure {
                        body: Rc::new(body.clone()),
                        param_names: param_names,
                        environment: env.clone()
                    })
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
                            (ff.function)(items.map(|v| eval(v, env)).collect())
                        }
                        x => panic!("{:?} is not executable", x)
                    }
                }
            }
        }
    }
}

