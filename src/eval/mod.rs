use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn, user_fn, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext, State};

mod environment;
mod foreign_function;
mod procedure;
mod context;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StepState {
    EvalThis(Value, bool),
    Return,
    Complete(Value),
    PreEvaluatedCallable {
        unevaluated: Vec<Value>
    },
    Lambda {
        func: Procedure,
        evaluated: Vec<Value>,
        unevaluated: Vec<Value>
    }
}

pub fn cleanup_stack<S: ?Sized + State>(prev_length: usize, ctx: &mut LoadedContext<S>) {
    while ctx.stack.len() > (prev_length - 2) {
        ctx.stack.pop();
    }
}

pub fn eval<S: ?Sized>(value: &Value, ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State
{
    let value = value.clone();

    ctx.stack.push(StepState::Return);
    ctx.stack.push(StepState::EvalThis(value, false));

    let len = ctx.stack.len();
    if let Err(e) = step_eval(ctx) {
        cleanup_stack(len, ctx);
        return Err(e);
    }

    while ctx.stack.len() > len {
        if let Err(e) = step_eval(ctx) {
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

fn step_eval<S: State + ?Sized>(ctx: &mut LoadedContext<S>) -> AresResult<()> {
    let top = ctx.stack.pop().unwrap();
    if let StepState::Complete(value) = top {
        match ctx.stack.pop().unwrap() {
            StepState::PreEvaluatedCallable { mut unevaluated } => {
                let procedure = match value {
                    Value::Lambda(procedure, _) => procedure,
                    Value::ForeignFn(func) => {
                        let apply_result = try!(apply_function(&func, unevaluated, ctx));
                        ctx.stack.push(StepState::Complete(apply_result));
                        return Ok(())
                    }
                    other => return Err(AresError::UnexecutableValue(other.clone())),
                };

                // we will be wanting the "next" element very often,
                // so reverse this right now and call `pop` to get the next one.
                unevaluated.reverse();
                if let Some(first) = unevaluated.pop() {
                    ctx.stack.push(StepState::Lambda {
                        func: procedure,
                        evaluated: vec![],
                        unevaluated: unevaluated,
                    });
                    ctx.stack.push(StepState::EvalThis(first, false));
                } else {
                    let returned = try!(apply_lambda(&procedure, vec![], ctx));
                    ctx.stack.push(StepState::Complete(returned));
                }
            }
            StepState::Lambda { func, mut evaluated, mut unevaluated } => {
                evaluated.push(value);
                if let Some(next) = unevaluated.pop() {
                    ctx.stack.push(StepState::Lambda {
                        func: func,
                        evaluated: evaluated,
                        unevaluated: unevaluated,
                    });
                    ctx.stack.push(StepState::EvalThis(next, false));
                } else {
                    let returned = try!(apply_lambda(&func, evaluated, ctx));
                    ctx.stack.push(StepState::Complete(returned));
                }
            }
            a => panic!("step_eval(..): invalid stack state: [..., {:?}, {:?}]",
                        a, StepState::Complete(value))
        }
    } else {
        match top {
            StepState::EvalThis(value, proc_head) => {
                try!(eval_this(value, ctx, proc_head));
            }
            StepState::Complete(_) => unreachable!(),
            a@StepState::Return |
            a@StepState::Lambda {..} |
            a@StepState::PreEvaluatedCallable { .. } =>
                panic!("step_eval(..): invalid stack state: [..., {:?}]", a)
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
            let lookup = ctx.env().borrow().get(symbol);
            match lookup {
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

        Value::List(items) => {
            let mut items: Vec<_> = (*items).clone();
            if items.len() == 0 {
                return Err(AresError::ExecuteEmptyList);
            }
            let first = items.remove(0);
            ctx.stack.push(StepState::PreEvaluatedCallable { unevaluated: items });
            ctx.stack.push(StepState::EvalThis(first, true));
            Ok(())
        }

        Value::Lambda(_, true) => Err(AresError::MacroReference),

        v => {
            ctx.stack.push(StepState::Complete(v));
            Ok(())
        },
    }
}

pub fn apply_lambda<S: ?Sized>(procedure: &Procedure,
                                  args: Vec<Value>,
                                  ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State {
    for arg in &args {
        if let &Value::ForeignFn(ForeignFunction { typ: FfType::Ast, .. }) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }

    let mut new_env = try!(procedure.gen_env(args));
    ctx.with_other_env(&mut new_env, |ctx| {
        let mut last = None;
        for body in &*procedure.bodies {
            // FIXME remove recursive call
            last = Some(try!(ctx.eval(body)));
        }
        // it's impossible to make a lambda without a body
        Ok(last.unwrap())
    })
}

pub fn apply_function<S: ?Sized>(function: &ForeignFunction<()>,
                                 args: Vec<Value>,
                                 ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State {
    for arg in &args {
        if let &Value::ForeignFn(ForeignFunction { typ: FfType::Ast, .. }) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }

    let corrected = try!(function.correct::<S>().or(Err(AresError::InvalidForeignFunctionState)));
    // FIXME pass the whole vec in
    (corrected.function)(&args[..], ctx)
}

pub fn apply<'a, S: ?Sized>(func: &Value,
                            args: Vec<Value>,
                            ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State {
    match func.clone() {
        Value::Lambda(procedure, _) => {
            apply_lambda(&procedure, args, ctx)
        }
        Value::ForeignFn(ff) => {
            apply_function(&ff, args, ctx)
        }
        other => Err(AresError::UnexecutableValue(other)),
    }
}
