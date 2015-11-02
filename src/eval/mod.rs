use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn, user_fn, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext, State};

mod environment;
mod foreign_function;
mod procedure;
mod context;

#[derive(Clone)]
pub enum StepState {
    EvalThis(Value, bool),
    PopEnv,
    Return,
    Complete(Value),
    PreEvaluatedCallable {
        unevaluated: Vec<Value>
    },
    ArgCollectingLambda {
        func: Procedure,
        evaluated: Vec<Value>,
        unevaluated: Vec<Value>
    },
    EvaluatingLambda {
        name: Option<String>,
        bodies: Vec<Value>,
    }
}

fn cleanup_stack<S: ?Sized + State>(prev_length: usize, ctx: &mut LoadedContext<S>) {
    while ctx.stack.len() > (prev_length - 2) {
        ctx.stack.pop();
    }
}

fn run_evaluation<S: ?Sized + State>(target_len: usize, ctx: &mut LoadedContext<S>) -> AresResult<(StepState, StepState)> {
    let starting_len = target_len;
    loop {
        let cur_len = ctx.stack.len();
        if cur_len < starting_len {
            panic!("run_evaluation(..): stack is lower than should be possible.");
        } else if cur_len == starting_len {
            if let Some(&StepState::Complete(_)) = ctx.stack.last() {
                break;
            }
        }

        if let Err(e) = step_eval(ctx) {
            cleanup_stack(starting_len, ctx);
            return Err(e);
        }
    }

    let top = ctx.stack.pop();
    let next_top = ctx.stack.pop();
    match (next_top, top) {
        (Some(next_top), Some(top)) => {
            Ok((next_top, top))
        }
        (next_top, top) =>
            panic!("run_evaluation(..): invalid stack state [..., {:?}, {:?}]", next_top, top),
    }
}

pub fn eval<S: ?Sized>(value: &Value, ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State
{
    let value = value.clone();

    ctx.stack.push(StepState::Return);
    ctx.stack.push(StepState::EvalThis(value, false));

    match try!(run_evaluation(ctx.stack.len(), ctx)) {
        (StepState::Return, StepState::Complete(value)) => Ok(value),
        (next_top, top) =>
            panic!("eval(..): invalid stack state [{:?}, {:?}, {:?}]", ctx.stack, next_top, top),
    }
}

pub fn apply<'a, S: ?Sized>(func: &Value,
                            args: Vec<Value>,
                            ctx: &mut LoadedContext<S>) -> AresResult<Value>
where S: State {
    let cur_len = ctx.stack.len();
    ctx.stack.push(StepState::Return);
    try!(do_apply(func.clone(), args, ctx));
    match try!(run_evaluation(cur_len + 2, ctx)) {
        (StepState::Return, StepState::Complete(value)) => Ok(value),
        (next_top, top) =>
            panic!("apply(..): invalid stack state [{:?}, {:?}, {:?}]", ctx.stack, next_top, top),
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
                        let apply_result = try!(apply_function(func, unevaluated, ctx));
                        ctx.stack.push(StepState::Complete(apply_result));
                        return Ok(())
                    }
                    other => return Err(AresError::UnexecutableValue(other.clone())),
                };

                // we will be wanting the "next" element very often,
                // so reverse this right now and call `pop` to get the next one.
                unevaluated.reverse();
                if let Some(first) = unevaluated.pop() {
                    ctx.stack.push(StepState::ArgCollectingLambda {
                        func: procedure,
                        evaluated: vec![],
                        unevaluated: unevaluated,
                    });
                    ctx.stack.push(StepState::EvalThis(first, false));
                } else {
                    try!(apply_lambda(procedure, vec![], ctx));
                }
            }
            StepState::ArgCollectingLambda { func, mut evaluated, mut unevaluated } => {
                evaluated.push(value);
                if let Some(next) = unevaluated.pop() {
                    ctx.stack.push(StepState::ArgCollectingLambda {
                        func: func,
                        evaluated: evaluated,
                        unevaluated: unevaluated,
                    });
                    ctx.stack.push(StepState::EvalThis(next, false));
                } else {
                    try!(apply_lambda(func, evaluated, ctx));
                }
            }
            StepState::EvaluatingLambda { mut bodies, name } => {
                if let Some(next_body) = bodies.pop() {
                    let body_eval = StepState::EvalThis(next_body, false);
                    let watching_state = StepState::EvaluatingLambda {
                        name: name,
                        bodies: bodies,
                    };
                    ctx.stack.push(watching_state);
                    ctx.stack.push(body_eval);
                } else {
                    ctx.stack.push(StepState::Complete(value));
                }
            }
            StepState::PopEnv => {
                ctx.env_stack.pop();
                ctx.stack.push(StepState::Complete(value));
            }
            a => panic!("step_eval(..): invalid stack state: [{:?}, {:?}, {:?}]",
                        ctx.stack, a, StepState::Complete(value))
        }
    } else {
        match top {
            StepState::EvalThis(value, proc_head) => {
                try!(eval_this(value, ctx, proc_head));
            }
            StepState::PopEnv => { ctx.env_stack.pop(); },
            StepState::Complete(_) => unreachable!(),
            a@StepState::Return |
            a@StepState::ArgCollectingLambda {..} |
            a@StepState::PreEvaluatedCallable { .. } |
            a@StepState::EvaluatingLambda { .. } =>
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

pub fn apply_lambda<S: ?Sized>(procedure: Procedure,
                                  args: Vec<Value>,
                                  ctx: &mut LoadedContext<S>) -> AresResult<()>
where S: State {
    for arg in &args {
        if let &Value::ForeignFn(ForeignFunction { typ: FfType::Ast, .. }) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }

    let new_env = try!(procedure.gen_env(args));
    let mut bodies = (*procedure.bodies).clone();
    bodies.reverse();

    let first_body = bodies.pop().unwrap();
    let body_eval = StepState::EvalThis(first_body, false);

    ctx.env_stack.push(new_env.clone());

    ctx.stack.push(StepState::PopEnv);
    if bodies.len() == 0 {
        // optimizing the common case (lambdas with only one body)
        ctx.stack.push(body_eval);
    } else {
        let watching_state = StepState::EvaluatingLambda {
            name: procedure.name,
            bodies: bodies,
        };
        ctx.env_stack.push(new_env.clone());
        ctx.stack.push(watching_state);
        ctx.stack.push(body_eval);
    }
    Ok(())
}

pub fn apply_function<S: ?Sized>(function: ForeignFunction<()>,
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

pub fn do_apply<'a, S: ?Sized>(func: Value, args: Vec<Value>, ctx: &mut LoadedContext<S>)
-> AresResult<()>
where S: State {
    match func {
        Value::Lambda(procedure, _) => {
            apply_lambda(procedure, args, ctx)
        }
        Value::ForeignFn(ff) => {
            let res = try!(apply_function(ff, args, ctx));
            ctx.stack.push(StepState::Complete(res));
            Ok(())
        }
        other => Err(AresError::UnexecutableValue(other)),
    }
}

impl ::std::fmt::Debug for StepState {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            &StepState::EvalThis(ref v, _) =>
                formatter.debug_tuple("EvalThis")
                         .field(v)
                         .finish(),
            &StepState::PopEnv =>
                formatter.debug_tuple("PopEnv")
                         .finish(),
            &StepState::Return =>
                formatter.debug_tuple("Return")
                         .finish(),
            &StepState::Complete(ref v) =>
                formatter.debug_tuple("Complete")
                         .field(v)
                         .finish(),
            &StepState::ArgCollectingLambda { ref func, ref evaluated, .. } =>
                formatter.debug_struct("ArgCollectingLambda")
                         .field("func", func)
                         .field("evaluated", evaluated)
                         .field("yet_to_be_evaluated", &"[..]")
                         .finish(),
            &StepState::PreEvaluatedCallable { ref unevaluated } =>
                formatter.debug_struct("PreEvaluatedCallable")
                         .field("unevaluated", unevaluated)
                         .finish(),
            &StepState::EvaluatingLambda { ref bodies, ref name, .. } =>
                formatter.debug_struct("EvaluatingLambda")
                         .field("name", name)
                         .field("bodies", bodies)
                         .field("env", &"{..}")
                         .finish()
        }
    }
}
