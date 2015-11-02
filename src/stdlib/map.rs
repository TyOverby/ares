use std::rc::Rc;
use std::collections::HashMap;

use {Value, AresResult, AresError, LoadedContext, State};

pub fn hash_map<S: State + ?Sized>(args: &[Value],
                                   ctx: &mut LoadedContext<S>)
                                   -> AresResult<Value> {
    if args.len() % 2 == 1 {
        return Err(AresError::UnexpectedArity {
            found: args.len() as u16,
            expected: "an even number".to_owned(),
        });
    }
    let mut m = HashMap::with_capacity(args.len() / 2);
    let mut key = None;
    for arg in args {
        match key {
            None => match try!(ctx.eval(arg)) {
                v@Value::List(_) | v@Value::Map(_) => {
                    return Err(AresError::UnexpectedType {
                        value: v,
                        expected: "a hashable type".to_owned(),
                    });
                }
                v => key = Some(v),
            },
            Some(k) => {
                let val = try!(ctx.eval(arg));
                m.insert(k, val);
                key = None
            }
        }
    }

    Ok(Value::Map(Rc::new(m)))
}
