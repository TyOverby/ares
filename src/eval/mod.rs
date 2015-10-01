use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext};

mod environment;
mod foreign_function;
mod procedure;
mod context;

pub fn eval(value: &Value, ctx: &mut LoadedContext) -> AresResult<Value> {
    match value {

        &Value::Ident(ref ident) => {
            match ctx.env().borrow().get(&ident) {
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

            match try!(eval(head, ctx)) {
                f@Value::Lambda(_) => {
                    let evald: AresResult<Vec<Value>> = items.iter().map(|v| ctx.eval(v)).collect();
                    let evald = try!(evald);
                    apply(&f, &evald[..], ctx)
                }

                f@Value::ForeignFn(_) => {
                        apply(&f, items, ctx)
                }
                x => Err(AresError::UnexecutableValue(x))
            }
        },

        &ref v => Ok(v.clone())

    }
}

pub fn apply<'a>(func: &Value, args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value>
{
    match func.clone() {
        Value::Lambda(procedure) => {
            let mut new_env = procedure.gen_env(args.iter().cloned());
            ctx.with_other_env(&mut new_env, |ctx| {
                let mut last = None;
                for body in &*procedure.bodies {
                    last = Some(try!(ctx.eval(body)));
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
