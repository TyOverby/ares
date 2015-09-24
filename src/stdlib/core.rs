use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, Environment, Procedure, AresResult, AresError, ParamBinding};

pub fn equals(args: &mut Iterator<Item=Value>) -> AresResult<Value> {
    let first = match args.next() {
        Some(v) => v,
        None => return Err(AresError::UnexpectedArity {
            found: 0,
            expected: "at least 2".into()
        })
    };
    let mut seen_2 = false;

    for next in args {
        seen_2 = true;
        if next != first {
            return Ok(Value::Bool(false))
        }
    }

    if !seen_2 {
        return Err(AresError::UnexpectedArity {
            found: 1,
            expected: "at least 2".into()
        });
    }

    Ok(Value::Bool(true))
}

pub fn lambda(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              _eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let param_names = match args.next() {
        Some(&Value::List(ref v)) => {
            let r: Vec<String> = v.iter().map(|n| {
                if let &Value::Ident(ref s) = n {
                    (&**s).clone()
                } else {
                    panic!("non ident param name");
                }
            }).collect();
            ParamBinding::ParamList(r)
        }
        Some(&Value::Ident(ref v)) => {
            ParamBinding::SingleIdent((**v).clone())
        }
        Some(x) => {
            return Err(AresError::UnexpectedArgsList(x.clone()));
        }
        None => {
            return Err(AresError::NoLambdaArgsList);
        }
    };

    let bodies:Vec<_> = args.cloned().collect();

    if bodies.len() == 0 {
        return Err(AresError::NoLambdaBody);
    }

    Ok(Value::Lambda(
            Procedure::new(
                None,
                Rc::new(bodies),
                param_names,
                env.clone())))
}

pub fn define(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let name: String = match args.next() {
        Some(&Value::Ident(ref s)) => (**s).clone(),
        Some(&ref other) => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Ident".into()
        }),
        None => return Err(AresError::NoNameDefine)
    };

    if env.borrow().is_defined_at_this_level(&name) {
        return Err(AresError::AlreadyDefined(name.into()))
    }

    let value = match args.next() {
        Some(v) => v,
        None => return Err(AresError::UnexpectedArity {
            found: 1,
            expected: "exactly 2".into()
        })
    };

    // TODO: check the remaining arg length and make a better error message
    if args.next().is_some() {
        return Err(AresError::UnexpectedArity {
            found: 3,
            expected: "exactly 2".into()
        })
    }

    let result = try!(eval(value, env));
    env.borrow_mut().insert(name, result.clone());
    Ok(result)
}

pub fn set(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let name = match args.next() {
        Some(&Value::Ident(ref s)) => (**s).clone(),
        Some(&ref v) => return Err(AresError::UnexpectedType {
            value: v.clone(),
            expected: "Ident".into()
        }),
        None => return Err(AresError::NoNameSet)
    };

    let value = args.next().unwrap();

    // TODO: check the remaining arg length and make a better error message
    if args.next().is_some() {
        return Err(AresError::UnexpectedArity {
            found: 3,
            expected: "exactly 2".into()
        })
    }

    if !env.borrow().is_defined(&name) {
        return Err(AresError::UndefinedName(name.into()))
    }

    let result = try!(eval(value, env));
    env.borrow_mut().insert(name, result.clone());
    Ok(result)
}

pub fn quote(args: &mut Iterator<Item=&Value>,
              _env: &Rc<RefCell<Environment>>,
              _eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let first = args.next().unwrap().clone();

    // TODO: check the remaining arg length and make a better error message
    if args.next().is_some() {
        return Err(AresError::UnexpectedArity {
            found: 2,
            expected: "exactly 1".into()
        })
    }
    Ok(first)
}

pub fn cond(args: &mut Iterator<Item=&Value>,
            env: &Rc<RefCell<Environment>>,
            eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let cond = args.next().unwrap();
    let true_branch = args.next().unwrap();
    let false_branch = args.next().unwrap();
    match try!(eval(cond, env)) {
        Value::Bool(true) => eval(true_branch, env),
        Value::Bool(false) => eval(false_branch, env),
        other => return Err(AresError::UnexpectedType {
            value: other,
            expected: "Bool".into()
        })
    }
}
