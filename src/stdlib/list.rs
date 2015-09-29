use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, Env, AresResult, AresError, free_fn};
use super::util::{no_more_or_arity_err, unwrap_or_arity_err};

pub fn build_list(args: &[Value],
                  env: &Env,
                  eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let mut args = args.iter();
    let vec = Rc::new(RefCell::new(Some(Vec::<Value>::new())));
    let writer = vec.clone();

    let func = move |values: &[Value]| -> AresResult<Value> {
        match &mut *writer.borrow_mut() {
            &mut Some(ref mut adder) => {
                let mut last = None;
                for value in values {
                    adder.push(value.clone());
                    last = Some(value);
                }

                match last {
                    Some(v) => Ok(v.clone()),
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

    let boxed_fn: Value = free_fn("add", func);

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

pub fn foreach(args: &[Value],
               env: &Env,
               eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let mut args = args.iter();
    let should_be_list = try!(unwrap_or_arity_err(args.next(), 0, "exactly 2"));
    let list = match try!(eval(should_be_list, env)) {
        Value::List(ref l) => l.clone(),
        other => return Err(AresError::UnexpectedType{
            value: other,
            expected: "List".into()
        }),
    };

    let func = try!(unwrap_or_arity_err(args.next().cloned(), 1, "exactly 2"));
    try!(no_more_or_arity_err(&mut args, 2, "exactly 2"));

    let mut count = 0;
    for element in list.iter() {
        let prog = Value::new_list(vec![func.clone(), element.clone()]);
        try!(eval(&prog, env));
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
