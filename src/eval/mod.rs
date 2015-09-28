use std::rc::Rc;
use std::cell::RefCell;

use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::Context;

mod environment;
mod foreign_function;
mod procedure;
mod context;

pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> AresResult<Value> {
    match value {
        &ref v@Value::String(_) => Ok(v.clone()),
        &ref v@Value::Int(_) => Ok(v.clone()),
        &ref v@Value::Float(_) => Ok(v.clone()),
        &ref v@Value::Bool(_) => Ok(v.clone()),

        &ref v@Value::ForeignFn(_) => Ok(v.clone()),
        &ref v@Value::Lambda(_) => Ok(v.clone()),

        &Value::Ident(ref ident) => {
            match env.borrow().get(&ident) {
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName((**ident).clone()))
            }
        }

        &Value::List(ref l) => {
            let mut items = l.iter();
            let head = match items.next() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList)
            };

            match try!(eval(head, env)) {
                Value::Lambda(procedure) => {
                    let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                    let evald = try!(evald);
                    let new_env = procedure.gen_env(evald.into_iter());
                    let mut last = None;
                    for body in &*procedure.bodies {
                        last = Some(try!(eval(body, &new_env)));
                    }
                    last.ok_or(AresError::NoLambdaBody)
                }
                Value::ForeignFn(ff) => {
                    match ff.function {
                        FfType::FreeFn(ff) => {
                            let evald: AresResult<Vec<Value>> = items.map(|v| eval(v, env)).collect();
                            let evald = try!(evald);
                            (ff)(&mut evald.into_iter())
                        }
                        FfType::UnEvalFn(uef) => (uef)(&mut items, env, &|v, e| eval(v, e))
                    }
                }
                x => Err(AresError::UnexecutableValue(x))
            }
        }
    }
}

