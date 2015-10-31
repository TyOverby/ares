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

pub fn cleanup_stack<S: ?Sized + State>(prev_length: usize, ctx: &mut LoadedContext<S>) {
    while ctx.stack.len() > (prev_length - 2) {
        ctx.stack.pop();
    }
}

pub fn eval<S: ?Sized>(value: &Value, ctx: &mut LoadedContext<S>, proc_head: bool) -> AresResult<Value>
where S: State
{
    let value = value.clone();

    ctx.stack.push(StepState::Return);
    ctx.stack.push(StepState::EvalThis(value));

    let len = ctx.stack.len();
    if let Err(e) = step_eval(ctx, proc_head) {
        cleanup_stack(len, ctx);
        return Err(e);
    }

    while ctx.stack.len() > len {
        if let Err(e) = step_eval(ctx, proc_head) {
            cleanup_stack(len, ctx);
            return Err(e);
        }
    }

    let result = ctx.stack.pop();
    let ret = ctx.stack.pop();
    match (ret, result) {
        (Some(StepState::Return), Some(StepState::Complete(value))) => {
            Ok(value)
        }
        (res, r) => panic!("eval(..): invalid stack state [..., {:?}, {:?}]", r, res),
    }
}

fn step_eval<S: State + ?Sized>(ctx: &mut LoadedContext<S>, proc_head: bool) -> AresResult<()> {
    let top = ctx.stack.pop().unwrap();
    if let StepState::Complete(_value) = top {
        let _what = ctx.stack.pop().unwrap();

    } else {
        match top {
            StepState::EvalThis(value) => {
                try!(eval_this(value, ctx, proc_head));
            }
            StepState::Complete(_) => unreachable!(),
            a@StepState::Return => panic!("step_eval(..): invalid stack state: [..., {:?}]", a)
        }
    }
    Ok(())
}

fn eval_this<S: State + ?Sized>(value: Value,
                               ctx: &mut LoadedContext<S>,
                               proc_head: bool)
                               -> AresResult<()> {
    match value {
        Value::Symbol(symbol) => {
            let func = ctx.env().borrow().get(symbol);
            match func {
                Some(Value::ForeignFn(ForeignFunction{typ: FfType::Ast, ..})) if !proc_head => {
                    Err(AresError::AstFunctionPass)
                }
                Some(v) => {
                    ctx.stack.push(StepState::Complete(v));
                    Ok(())
                },
                None => Err(AresError::UndefinedName(ctx.interner().lookup_or_anon(symbol))),
            }
        }

        Value::List(ref items) => {
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
                    let apply_result = try!(apply(&f, &evald[..], ctx));
                    ctx.stack.push(StepState::Complete(apply_result));
                    Ok(())
                }

                f@Value::ForeignFn(_) => {
                    let apply_result = try!(apply(&f, items, ctx));
                    ctx.stack.push(StepState::Complete(apply_result));
                    Ok(())

                }
                x => Err(AresError::UnexecutableValue(x)),
            }
        }

        Value::Lambda(_, true) => Err(AresError::MacroReference),

        v => {
            ctx.stack.push(StepState::Complete(v));
            Ok(())
        },
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
