use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, Environment, Procedure, AresResult, AresError, ParamBinding};
use super::util::{unwrap_or_arity_err, no_more_or_arity_err};

pub fn equals(args: &mut Iterator<Item=Value>) -> AresResult<Value> {
    let first = try!(unwrap_or_arity_err(args.next(), 0, "at least 2"));
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
    let param_names = match try!(unwrap_or_arity_err(args.next(), 0, "2 or more")) {
        &Value::List(ref v) => {
            let r: AresResult<Vec<String>> = v.iter().map(|n| {
                match n {
                    &Value::Ident(ref s) => Ok((&**s).clone()),
                    &ref other => Err(AresError::UnexpectedType {
                        value: other.clone(),
                        expected: "Ident".into()
                    })
                }
            }).collect();
            ParamBinding::ParamList(try!(r))
        }
        &Value::Ident(ref v) => {
            ParamBinding::SingleIdent((**v).clone())
        }
        x => {
            return Err(AresError::UnexpectedArgsList(x.clone()));
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
    let name: String = match try!(unwrap_or_arity_err(args.next(), 0, "exactly 2")) {
        &Value::Ident(ref s) => (**s).clone(),
        &ref other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Ident".into()
        }),
    };

    if env.borrow().is_defined_at_this_level(&name) {
        return Err(AresError::AlreadyDefined(name.into()))
    }

    let value = try!(unwrap_or_arity_err(args.next(), 1, "exactly 2"));

    try!(no_more_or_arity_err(args, 2, "exactly 2"));

    let result = try!(eval(value, env));
    env.borrow_mut().insert(name, result.clone());
    Ok(result)
}

pub fn set(args: &mut Iterator<Item=&Value>,
              env: &Rc<RefCell<Environment>>,
              eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let name = match try!(unwrap_or_arity_err(args.next(), 0, "exactly 2")) {
        &Value::Ident(ref s) => (**s).clone(),
        &ref v => return Err(AresError::UnexpectedType {
            value: v.clone(),
            expected: "Ident".into()
        }),
    };

    let value = try!(unwrap_or_arity_err(args.next(), 1, "exactly 2"));

    try!(no_more_or_arity_err(args, 2, "exactly 2"));

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
    let first = try!(unwrap_or_arity_err(args.next().cloned(), 0, "exactly 1"));
    try!(no_more_or_arity_err(args, 1, "exactly 1"));
    Ok(first)
}

pub fn cond(args: &mut Iterator<Item=&Value>,
            env: &Rc<RefCell<Environment>>,
            eval: &Fn(&Value, &Rc<RefCell<Environment>>) -> AresResult<Value>) -> AresResult<Value> {
    let (cond, true_branch, false_branch) =
        (try!(unwrap_or_arity_err(args.next(), 0, "exactly 3")),
         try!(unwrap_or_arity_err(args.next(), 1, "exactly 3")),
         try!(unwrap_or_arity_err(args.next(), 2, "exactly 3")));

    match try!(eval(cond, env)) {
        Value::Bool(true) => eval(true_branch, env),
        Value::Bool(false) => eval(false_branch, env),
        other => Err(AresError::UnexpectedType {
            value: other,
            expected: "Bool".into()
        })
    }
}
