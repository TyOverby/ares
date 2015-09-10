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

pub fn eval(value: Value, env: &Rc<RefCell<Environment>>) -> Value {
    match value {
        v@Value::String(_) => v,
        v@Value::Int(_) => v,
        v@Value::Float(_) => v,
        v@Value::Bool(_) => v,

        v@Value::ForeignFn(_) => v,
        v@Value::Lambda(_) => v,

        Value::Ident(ident) => {
            match env.borrow().get(&ident) {
                Some(env) => env,
                None => panic!("Variable {} not found", ident)
            }
        }

        Value::List(mut l) => {
            match &*l.remove(0) {
                &Value::Ident(ref v) if AsRef::<str>::as_ref(v) == "quote" => {
                    (&*l.into_iter().next().unwrap()).clone()
                }
                &Value::Ident(ref v) if AsRef::<str>::as_ref(v) == "lambda" => {
                    let names = l.remove(0);
                    let body = l.remove(0);

                    let param_names = match &*names {
                        &Value::List(ref v) => {
                            v.iter().map(|n| {
                                if let &Value::Ident(ref s) = &**n {
                                    s.clone()
                                } else {
                                    panic!("non ident param name");
                                }
                            }).collect()
                        }
                        _ => panic!("no param names list found for lambda")
                    };

                    Value::Lambda(Procedure {
                        body: body,
                        param_names: param_names,
                        environment: env.clone()
                    })
                }
                &Value::Ident(ref v) if AsRef::<str>::as_ref(v) == "define" => {
                    let name = if let &Value::String(ref s) = &* l.remove(0) {
                        s.clone()
                    } else {
                        panic!("define with no name");
                    };
                    let value: Value = (*l.remove(0)).clone();
                    let result = eval(value, env);
                    env.borrow_mut().bindings.insert(name, result.clone());
                    result
                }
                &Value::Ident(ref v) if AsRef::<str>::as_ref(v) == "if" => {
                    match eval((*l.remove(0)).clone(), env) {
                        Value::Bool(true) => eval((*l.remove(0)).clone(), env),
                        Value::Bool(false) => eval((*l.remove(1)).clone(), env),
                        _ => panic!("boolean expected in 'if'")
                    }
                }
                &ref other => {
                    match eval(other.clone(), env) {
                        Value::Lambda(procedure) => {
                            let new_env = procedure.gen_env(l.into_iter().map(|v| eval((*v).clone(), env)));
                            eval((*procedure.body).clone(), &new_env)
                        }
                        Value::ForeignFn(ff) => {
                            (ff.function)(l.into_iter().map(|v| eval((*v).clone(), env)).collect())
                        }
                        x => panic!("{:?} is not executable", x)
                    }
                }
            }
        }
    }
}

