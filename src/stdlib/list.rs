use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, Env, AresResult, AresError, ForeignFunction, apply};
use super::util::{no_more_or_arity_err, unwrap_or_arity_err};

pub fn build_list(args: &mut Iterator<Item=&Value>,
                  env: &Env,
                  eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let vec = Rc::new(RefCell::new(Some(Vec::<Value>::new())));
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
    let should_be_list = try!(unwrap_or_arity_err(args.next(), 0, "exactly 2"));
    let list = match try!(eval(should_be_list, env)) {
        Value::List(ref l) => l.clone(),
        other => return Err(AresError::UnexpectedType{
            value: other,
            expected: "List".into()
        }),
    };

    let func = try!(unwrap_or_arity_err(args.next().cloned(), 1, "exactly 2"));
    try!(no_more_or_arity_err(args, 2, "exactly 2"));
    let func = try!(eval(&func, env));

    let mut count = 0;
    for element in list.iter() {
        let mut singleton_iterator = Some(element).into_iter();
        try!(apply(&func, &mut singleton_iterator, env));
        count += 1;
    }

    Ok(Value::Int(count))
}

pub static LIST: &'static str = "(lambda list list)";

pub static MAP: &'static str =
"(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (push (fn element)))))))";

pub static FOLD_LEFT: &'static str =
"(lambda (list default fn)
    (for-each list (lambda (element)
        (set default (fn default element))
    ))
    default)";

pub static FILTER: &'static str =
"(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (if (fn element)
                    (push element)
                    false))))))";

pub static CONCAT: &'static str =
"(lambda lists
    (build-list
        (lambda (push)
            (for-each lists (lambda (list)
                (for-each list (lambda (element)
                    (push element))))))))";
