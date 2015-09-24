use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use ::{Value, Environment, Env, Procedure, AresResult, AresError, ForeignFunction};

pub fn build_list(args: &mut Iterator<Item=&Value>,
              env: &Env,
              eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let vec = Rc::new(RefCell::new(Some(Vec::<Value>::new())));
    let new_env = Environment::new_with_data(env.clone(), HashMap::new());
    let writer = vec.clone();
    let func = move |values: &mut Iterator<Item=Value>| {
        match &mut *writer.borrow_mut() {
            &mut Some(ref mut adder) => {
                let mut last = None;
                for value in values {
                    adder.push(value.clone());
                    last = Some(value);
                }

                match last {
                    Some(v) => Ok(v),
                    None => Err(AresError::UnexpectedArity {
                        found: 0,
                        expected: "at least 1".to_string()
                    })
                }
            },
            &mut None => {
                let err_msg = "build-list adder called after completion of build-list.";
                return Err(AresError::InvalidState(err_msg.to_string()))
           }
        }
    };

    let boxed_fn = ForeignFunction::new_free_function("add".into(), Rc::new(func));
    let boxed_fn = Value::ForeignFn(boxed_fn);

    let evaluator = match args.next() {
        Some(lambda) => lambda.clone(),
        None => {
            return Err(AresError::UnexpectedArity {
                found: 0,
                expected: "exactly 1".into()
            });
        }
    };

    let rest = args.count();
    if rest != 0 {
        return Err(AresError::UnexpectedArity {
            found: rest as u16 + 1,
            expected: "exactly 1".to_string()
        });
    }

    try!(eval(&Value::new_list(vec![evaluator, boxed_fn]), env));

    let mut v = vec.borrow_mut();
    Ok(Value::new_list(v.take().unwrap()))
}

pub fn foreach(args: &mut Iterator<Item=&Value>,
              env: &Env,
              eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let list = match args.next() {
        Some(&Value::List(ref l)) => l.clone(),
        Some(v) => return Err(AresError::UnexpectedType{
            value: v.clone(),
            expected: "List".into()
        }),
        None => return Err(AresError::UnexpectedArity {
            found: 0,
            expected: "exactly 2".into()
        })
    };

    let func = match args.next() {
        Some(f) => f.clone(),
        None => return Err(AresError::UnexpectedArity {
            found: 1,
            expected: "exactly 2".into()
        })
    };

    let mut count = 0;
    for element in list.iter() {
        let prog = Value::new_list(vec![func.clone(), element.clone()]);
        try!(eval(&prog, env));
    }

    Ok(Value::Int(count))
}

pub static LIST: &'static str = "(lambda l l)";

pub static MAP: &'static str = "(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (push (fn element)))))))";

pub static FOLD_LEFT: &'static str =
"(lambda (default list fn)
    (for-each list (lambda (element)
        (set! default (fn default element))
    ))
    default)";

pub static FILTER: &'static str = "(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (if (fn element)
                    (push element)
                    false))))))";
