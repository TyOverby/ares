use std::rc::Rc;
use std::collections::HashMap;

use ::{Value, AresResult, AresError, LoadedContext};

pub fn hash_map(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
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
            None => key = Some(try!(ctx.eval(arg))),
            Some(k) => {
                let val = try!(ctx.eval(arg));
                m.insert(k, val);
                key = None
            }
        }
    }

    Ok(Value::Map(Rc::new(m)))
}
