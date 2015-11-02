use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment, BindingHashMap};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn, user_fn, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext, State};

mod environment;
mod foreign_function;
mod procedure;
mod context;

pub fn eval<S: State + ?Sized>(value: &Value,
                               ctx: &mut LoadedContext<S>,
                               proc_head: bool)
                               -> AresResult<Value> {
    match value {
        &Value::Symbol(symbol) => {
            match ctx.env().borrow().get(symbol) {
                Some(Value::ForeignFn(ForeignFunction{typ: FfType::Ast, ..})) if !proc_head => {
                    Err(AresError::AstFunctionPass)
                }
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName(ctx.interner().lookup_or_anon(symbol))),
            }
        }

        &Value::List(ref items) => {
            let head = match items.first() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList),
            };
            let items = &items[1..];

            match try!(eval(head, ctx, true)) {
                Value::Lambda(_, true) => Err(AresError::MacroReference),
                f@Value::Lambda(_, _) => {
                    let evald: AresResult<Vec<Value>> = items.iter().map(|v| ctx.eval(v)).collect();
                    let evald = try!(evald);
                    apply(&f, &evald[..], ctx)
                }

                f@Value::ForeignFn(_) => {
                    apply(&f, items, ctx)
                }
                x => Err(AresError::UnexecutableValue(x)),
            }
        }

        &Value::Lambda(_, true) => Err(AresError::MacroReference),

        &ref v => Ok(v.clone()),
    }
}

pub fn apply<'a, S: State + ?Sized>(func: &Value,
                                    args: &[Value],
                                    ctx: &mut LoadedContext<S>)
                                    -> AresResult<Value> {
    for arg in args {
        if let &Value::ForeignFn(ForeignFunction{typ: FfType::Ast, ..}) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }

    match func.clone() {
        Value::Lambda(procedure, _) => {
            let mut new_env = try!(procedure.gen_env(args.iter().cloned()));
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
            let corrected = try!(ff.correct::<S>().or(Err(AresError::InvalidForeignFunctionState)));
            (corrected.function)(args, ctx)
        }
        other => Err(AresError::UnexecutableValue(other.clone())),
    }
}
