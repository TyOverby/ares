use std::rc::Rc;
use std::collections::HashMap;

use ::{Value, AresResult, Env, AresError};

pub fn hash_map(args: &[Value],
                env: &Env,
                eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    if args.len() % 2 == 1 {
        return Err(AresError::UnexpectedArity {
            found: args.len() as u16,
            expected: "an even number".to_owned()
        })
    }
    let mut m = HashMap::with_capacity(args.len() / 2);
    let mut key = None;
    for arg in args {
        match key {
            None => key = Some(try!(eval(arg, env))),
            Some(k) => {
                let val = try!(eval(arg, env));
                m.insert(k, val);
                key = None
            }
        }
    }
    Ok(Value::Map(Rc::new(m)))
}
