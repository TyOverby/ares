use std::rc::Rc;
use std::cell::RefCell;

use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext};

mod environment;
mod foreign_function;
mod procedure;
mod context;

pub fn eval<S>(value: &Value<S>, ctx: &mut LoadedContext<S>) -> AresResult<Value<S>, S> {
    match value {
        &ref v@Value::String(_) => Ok(v.clone()),
        &ref v@Value::Int(_) => Ok(v.clone()),
        &ref v@Value::Float(_) => Ok(v.clone()),
        &ref v@Value::Bool(_) => Ok(v.clone()),
        &ref v@Value::ForeignFn(_) => Ok(v.clone()),
        &ref v@Value::Lambda(_) => Ok(v.clone()),

        &Value::Ident(ref ident) => {
            match ctx.env().borrow().get(&ident) {
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName((**ident).clone()))
            }
        }

        &Value::List(ref items) => {
            let head = match items.first() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList)
            };
            let items = &items[1..];

            match try!(eval(head, ctx)) {
                f@Value::Lambda(_) => {
                    let evald: AresResult<Vec<Value<S>>, S> = items.iter().map(|v| ctx.eval(v)).collect();
                    let evald = try!(evald);
                    apply(&f, &evald[..], ctx)
                }

                f@Value::ForeignFn(_) => {
                        apply(&f, items, ctx)
                }
                x => Err(AresError::UnexecutableValue(x))
            }
        }
    }
}

pub fn apply<'a, S>(func: &Value<S>, args: &[Value<S>], ctx: &mut LoadedContext<'a, S>) -> AresResult<Value<S>, S>
{
    match func.clone() {
        Value::Lambda(procedure) => {
            let new_env = procedure.gen_env(args.iter().cloned());
            ctx.with_env(new_env, move |new_ctx| {
                let mut last = None;
                for body in &*procedure.bodies {
                    last = Some(try!(eval(body, new_ctx)));
                }
                // it's impossible to make a lambda without a body
                Ok(last.unwrap())
            })
        }
        Value::ForeignFn(ff) => {
            (ff.function)(args, ctx)
        }
        other => Err(AresError::UnexecutableValue(other.clone()))
    }
}
