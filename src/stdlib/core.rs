use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, Environment, Procedure};


pub fn lambda(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              _eval: fn(&Value, &Rc<RefCell<Environment>>) -> Value) -> Value {
    let names = args.next().unwrap();
    let bodies  = args.cloned().collect();
    let param_names = match names {
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

    Value::Lambda(Procedure::new(Rc::new(bodies),
                                 param_names, env.clone()))
}

pub fn define(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              eval: fn(&Value, &Rc<RefCell<Environment>>) -> Value) -> Value {
    let name = match args.next().unwrap() {
        &Value::Ident(ref s) => (&**s).clone(),
        & ref other => panic!("define with no name: {:?}", other)
    };
    let value = args.next().unwrap();

    if args.next().is_some() {
        panic!("define with more than 2 args");
    }

    let result = eval(value, env);
    env.borrow_mut().insert(name, result.clone());
    result
}

pub fn quote(args: &mut Iterator<Item=&Value>,
              _env: &Rc<RefCell<Environment>>,
              _eval: fn(&Value, &Rc<RefCell<Environment>>) -> Value) -> Value {
    let first = args.next().unwrap().clone();
    if args.next().is_some() {
        panic!("Multiple arguments to quote");
    }
    first
}

pub fn cond(args: &mut Iterator<Item=&Value>,
            env: &Rc<RefCell<Environment>>,
            eval: fn(&Value, &Rc<RefCell<Environment>>) -> Value) -> Value {
    let true_cond = args.next().unwrap();
    let false_cond = args.next().unwrap();
    match eval(args.next().unwrap(), env) {
        Value::Bool(true) => eval(true_cond, env),
        Value::Bool(false) => eval(false_cond, env),
        _ => panic!("boolean expected in 'if'")
    }
}
