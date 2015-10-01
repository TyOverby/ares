use std::rc::Rc;
use std::cell::RefCell;

use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::Context;

mod environment;
mod foreign_function;
mod procedure;
mod context;

pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> AresResult<Value> {
    match value {

        &Value::Ident(ref ident) => {
            match env.borrow().get(&ident) {
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName((**ident).clone()))
            }
        },

        &Value::List(ref items) => {
            let head = match items.first() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList)
            };
            let items = &items[1..];

            match try!(eval(head, env)) {
                f@Value::Lambda(_) => {
                    let evald: AresResult<Vec<Value>> = items.iter().map(|v| eval(v, env)).collect();
                    let evald = try!(evald);
                    apply(&f, &evald[..], env)
                }

                f@Value::ForeignFn(_) => {
                        apply(&f, items, env)
                }
                x => Err(AresError::UnexecutableValue(x))
            }
        },

        &ref v => Ok(v.clone())

    }
}

pub fn apply<'a>(func: &Value, args: &[Value], env: &'a Env) -> AresResult<Value>
{
    match func.clone() {
        Value::Lambda(procedure) => {
            let new_env = procedure.gen_env(args.iter().cloned());
            let mut last = None;
            for body in &*procedure.bodies {
                last = Some(try!(eval(body, &new_env)));
            }
            // it's impossible to make a lambda without a body
            Ok(last.unwrap())
        }
        Value::ForeignFn(ff) => {
            (ff.function)(args, env, &|a, b| eval(a, b))
        }
        other => Err(AresError::UnexecutableValue(other.clone()))
    }
}
