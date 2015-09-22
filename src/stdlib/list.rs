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
                let val = match values.next() {
                    Some(v) => {
                        println!("pushing {:?}", v);
                        adder.push(v.clone());
                        Ok(v)
                    }
                    None => {
                        Err(AresError::UnexpectedArity {
                           found: 0,
                           expected: "exactly 1".to_string()
                        })
                    }
                };

                let rest = values.count();
                if rest > 0 {
                    return Err(AresError::UnexpectedArity {
                        found: rest as u16 + 1,
                        expected: "exactly 1".to_string()
                    });
                }
                val
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
