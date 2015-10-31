use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn, user_fn, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext, State};

mod environment;
mod foreign_function;
mod procedure;
mod context;

#[derive(Debug)]
pub enum StepState {
    EvalThis(Value),
    Return,
    Complete(Value)
}

pub fn eval<S: ?Sized>(value: &Value, ctx: &mut LoadedContext<S>, proc_head: bool) -> AresResult<Value>
where S: State
{
    let value = value.clone();

    ctx.stack.push(StepState::Return);
    ctx.stack.push(StepState::EvalThis(value));

    let len = ctx.stack.len();
    try!(step_eval(ctx, proc_head));
    while ctx.stack.len() > len {
        try!(step_eval(ctx, proc_head));
    }

    let result = ctx.stack.pop().unwrap();
    let ret = ctx.stack.pop().unwrap();
    match (ret, result) {
        (StepState::Return, StepState::Complete(value)) => {
            Ok(value)
        }
        (res, r) => panic!("eval(..): invalid stack state [..., {:?}, {:?}]", r, res),
    }
}

fn step_eval<S: State + ?Sized>(ctx: &mut LoadedContext<S>, proc_head: bool) -> AresResult<()> {
    let top = ctx.stack.pop().unwrap();
    if let StepState::Complete(value) = top {
        let what = ctx.stack.pop().unwrap();

    } else {
        match top {
            StepState::EvalThis(value) => {
                let result = try!(eval_this(&value, ctx, proc_head));
                ctx.stack.push(StepState::Complete(result));
            }
            StepState::Complete(_) => unreachable!(),
            a@StepState::Return => panic!("step_eval(..): invalid stack state: [..., {:?}]", a)
        }
    }
    Ok(())
}

fn eval_this<S: State + ?Sized>(value: &Value,
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
