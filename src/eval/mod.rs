use super::{Value, AresError, AresResult};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, free_fn, ast_fn, user_fn, FfType};
pub use self::procedure::{Procedure, ParamBinding};
pub use self::context::{Context, LoadedContext, State};

mod environment;
mod foreign_function;
mod procedure;
mod context;
mod transformations;

#[derive(Clone)]
pub enum StepState {
    EvalThis(Value, bool),
    PopEnv,
    Return,
    Complete(Value),
    PreEvaluatedCallable {
        unevaluated: Vec<Value>,
    },
    ArgCollectingLambda {
        procedure: Procedure,
        evaluated: Vec<Value>,
        unevaluated: Vec<Value>,
    },
    EvaluatingLambda {
        name: Option<String>,
        bodies: Vec<Value>,
    },
}

fn cleanup_stack<S: ?Sized + State>(target_size: usize, ctx: &mut LoadedContext<S>) {
    while ctx.stack.len() > target_size {
        ctx.stack.pop();
    }
}

/// Runs the eval-loop until the state stack of of a target size.
/// The target size is usually the size of the stack before the call to eval/apply
/// plus 2 (one spot for the StepState::Return, one spot for the StepState::Complete)
fn run_evaluation<S: ?Sized + State>(target_len: usize,
                                     cleanup_len: usize,
                                     ctx: &mut LoadedContext<S>)
                                     -> AresResult<(StepState, StepState)> {
    loop {
        let cur_len = ctx.stack.len();
        // If we drop below the target_len then something has gone terribly wrong.
        if cur_len < target_len {
            panic!("run_evaluation(..): stack is lower than should be possible.");
        } else if cur_len == target_len {
            // Sometimes we could be at the target_len, but the topmost state is
            // something like a `StepState::EvalThis`.  In which case, we need
            // to keep running the loop.
            if let Some(&StepState::Complete(_)) = ctx.stack.last() {
                break;
            }
        }

        // Make one step on the interpreter
        let result = step_eval(ctx);

        // If an error occurred, clean up the stack from this point and propogate
        // the error upwards.
        if let Err(e) = result {
            cleanup_stack(cleanup_len, ctx);
            return Err(e);
        }
    }

    // Once the eval-loop is done, we are interested in the top-two elements on the
    // stack.
    // The `top` should contain the `StepState::Complete` result.
    // The `next_top` should contain `StepState::Return`.
    let top = ctx.stack.pop();
    let next_top = ctx.stack.pop();
    match (next_top, top) {
        (Some(next_top), Some(top)) => {
            Ok((next_top, top))
        }
        (next_top, top) => panic!("run_evaluation(..): invalid stack state [..., {:?}, {:?}]",
                                  next_top,
                                  top),
    }
}

pub fn eval<S: ?Sized>(value: &Value, ctx: &mut LoadedContext<S>) -> AresResult<Value>
    where S: State
{
    // FIXME: this will require an interface change that I don't want to do right
    // now.
    let value = value.clone();

    let cleanup_len = ctx.stack.len();

    // Push the return signal and a request to evaluate the value onto the stack.
    ctx.stack.push(StepState::Return);
    ctx.stack.push(StepState::EvalThis(value, false));

    match try!(run_evaluation(ctx.stack.len(), cleanup_len, ctx)) {
        (StepState::Return, StepState::Complete(value)) => Ok(value),
        (next_top, top) => panic!("eval(..): invalid stack state [{:?}, {:?}, {:?}]",
                                  ctx.stack,
                                  next_top,
                                  top),
    }
}

pub fn apply<'a, S: ?Sized>(func: &Value,
                            args: Vec<Value>,
                            ctx: &mut LoadedContext<S>)
                            -> AresResult<Value>
    where S: State
{
    // Keep track of the current stack size.
    let prior_len = ctx.stack.len();
    // Push the return signal onto the stack.
    ctx.stack.push(StepState::Return);
    // `do_apply` will push either 1, 2, or 3 items on the stack by itself.
    try!(do_apply(func.clone(), args, ctx));
    // Run the evaluation with a target end point of the prior length + 2
    // (one for the return, one for the Completed value.
    match try!(run_evaluation(prior_len + 2, prior_len, ctx)) {
        (StepState::Return, StepState::Complete(value)) => Ok(value),
        (next_top, top) => panic!("apply(..): invalid stack state [{:?}, {:?}, {:?}]",
                                  ctx.stack,
                                  next_top,
                                  top),
    }
}

/// Moves the interpreter one "step" forward in the execution.
fn step_eval<S: State + ?Sized>(ctx: &mut LoadedContext<S>) -> AresResult<()> {
    // Pop the top off of the top value in the stack and switch on the value
    // contained within.
    let top = ctx.stack.pop().unwrap();
    // If a value was just computed, we need to apply that computed value to the
    // state machine that
    // is just below.
    if let StepState::Complete(value) = top {
        match ctx.stack.pop().unwrap() {
            StepState::PreEvaluatedCallable { unevaluated } => {
                // A PreEvaluatedCallable just got the "function" evaluated.
                // `value` is the function that will eventually be called
                try!(transformations::from_pre_evaluated(unevaluated, value, ctx));
            }
            StepState::ArgCollectingLambda { procedure, unevaluated, evaluated} => {
                // An ArgCollectingLambda just got one of its arguments evaluated.
                // `value` is the post-evalauted argument.
                try!(transformations::from_arg_collecting_lambda(procedure,
                                                                 unevaluated,
                                                                 evaluated,
                                                                 value,
                                                                 ctx));
            }
            StepState::EvaluatingLambda { bodies, name } => {
                // An EvaluatingLambda just got the result from the execution of one of its
                // bodies. `value` is the retult of that body being executed.
                try!(transformations::from_evaluating_lambda(bodies, name, value, ctx));
            }
            StepState::PopEnv => {
                // Ok, this one isn't a state machine.  If you see a PopEnv, just
                // pop the env-stack and push the completed value back on the stack.
                ctx.env_stack.pop();
                ctx.stack.push(StepState::Complete(value));
            }
            // All of these should be impossible to reach, so let's panic.
            a@StepState::EvalThis(_, _) |
            a@StepState::Return |
            a@StepState::Complete(_) =>
                panic!("step_eval(..): invalid stack state: [{:?}, {:?}, {:?}]",
                       ctx.stack,
                       a,
                       StepState::Complete(value)),
        }
    } else {
        match top {
            // We just checked for Complete above.
            StepState::Complete(_) => unreachable!(),
            StepState::EvalThis(value, proc_head) => {
                // Forward to eval_this, which contains most of what used to be the `eval`
                // function.
                try!(eval_this(value, ctx, proc_head));
            }
            StepState::PopEnv => {
                ctx.env_stack.pop();
            }
            // These should all be impossible to reach.
            a@StepState::Return |
            a@StepState::ArgCollectingLambda {..} |
            a@StepState::PreEvaluatedCallable { .. } |
            a@StepState::EvaluatingLambda { .. } =>
                panic!("step_eval(..): invalid stack state: [..., {:?}]", a),
        }
    }
    Ok(())
}

/// Prepares a value to be evaluated based.
///
/// proc_head is true when the `value` being evaluated is located at
/// the head of a procedure call.
/// It is false when `value` is an argument.
fn eval_this<S: State + ?Sized>(value: Value,
                                ctx: &mut LoadedContext<S>,
                                proc_head: bool)
                                -> AresResult<()> {
    match value {
        Value::Symbol(symbol) => {
            let lookup = ctx.env().borrow().get(symbol);
            match lookup {
                // Ban Ast functions that are getting passed as arguments.
                Some(Value::ForeignFn(ForeignFunction{typ: FfType::Ast, ..})) if !proc_head => {
                    Err(AresError::AstFunctionPass)
                }
                Some(v) => {
                    ctx.stack.push(StepState::Complete(v));
                    Ok(())
                }
                None => Err(AresError::UndefinedName(ctx.interner().lookup_or_anon(symbol))),
            }
        }

        Value::List(items) => {
            let mut items: Vec<_> = (*items).clone();
            if items.len() == 0 {
                return Err(AresError::ExecuteEmptyList);
            }
            // Grab the first element in the list.  This is the function that is
            // going to be called.
            //
            // (a b c d)
            //  ^
            let first = items.remove(0);
            // Start a pre-evaluated callable with the rest of the items.
            ctx.stack.push(StepState::PreEvaluatedCallable { unevaluated: items });
            // Try to evaluate the head.
            ctx.stack.push(StepState::EvalThis(first, true));
            Ok(())
        }

        Value::Lambda(_, true) => Err(AresError::MacroReference),

        v => {
            // Any other value is already evaluated, so we push that back on the
            // stack as being completed.
            ctx.stack.push(StepState::Complete(v));
            Ok(())
        }
    }
}

fn apply_lambda<S: ?Sized>(procedure: Procedure,
                           args: Vec<Value>,
                           ctx: &mut LoadedContext<S>)
                           -> AresResult<()>
    where S: State
{
    // Make sure that there weren't any raw AST functions being passed in to the
    // lambda.
    for arg in &args {
        if let &Value::ForeignFn(ForeignFunction { typ: FfType::Ast, .. }) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }


    let mut bodies = (*procedure.bodies).clone();
    // Reverse the body order because we'll be using pop to get them off and we
    // want them in the correct order.
    bodies.reverse();

    // Unwrap is ok because lambdas must have at least one body.
    let first_body = bodies.pop().unwrap();

    // TODO: in the future, we might be able to skip all of this
    // if there aren't any args and there aren't any `define`s in the lambda body.
    //
    // Generate the new environment for the duration of the lambda body execution.
    // This environment will be pushed on the env-stack and then popped off once all
    // the bodies are done executing.
    let new_env = try!(procedure.gen_env(args));

    // Push the new environment on the env-stack, and immediately push the PopEnv on
    // the step-state stack.  When the lambda is done being executed, the PopEnv
    // will be on the top of the stack, so this env what we just pushed on to the
    // env-stack will be popped off.
    ctx.env_stack.push(new_env.clone());
    ctx.stack.push(StepState::PopEnv);

    // A call to evaluate the first body under the new environment.
    let body_eval = StepState::EvalThis(first_body, false);
    if bodies.len() == 0 {
        // Optimizing the common case (lambdas with only one body).
        // This case doesn't need an EvaluatingLambda on the stack because
        // there are no further bodies to evaluate.
        ctx.stack.push(body_eval);
    } else {
        // Make a watching state that holds the rest of the bodies of the lambda.
        let watching_state = StepState::EvaluatingLambda {
            name: procedure.name,
            bodies: bodies,
        };
        ctx.stack.push(watching_state);
        ctx.stack.push(body_eval);
    }
    Ok(())
}

fn apply_function<S: ?Sized>(function: ForeignFunction<()>,
                             args: Vec<Value>,
                             ctx: &mut LoadedContext<S>)
                             -> AresResult<Value>
    where S: State
{
    // Make sure that there weren't any raw AST functions being passed in to the
    // function.
    for arg in &args {
        if let &Value::ForeignFn(ForeignFunction { typ: FfType::Ast, .. }) = arg {
            return Err(AresError::AstFunctionPass);
        }
    }

    // Translate the function back into the correct generic form.
    let corrected = try!(function.correct::<S>().or(Err(AresError::InvalidForeignFunctionState)));
    // Call the function
    // FIXME pass the whole vec in
    (corrected.function)(&args[..], ctx)
}

fn do_apply<'a, S: ?Sized>(func: Value,
                           args: Vec<Value>,
                           ctx: &mut LoadedContext<S>)
                           -> AresResult<()>
    where S: State
{
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
            &StepState::EvalThis(ref v, _) => formatter.debug_tuple("EvalThis")
                                                       .field(v)
                                                       .finish(),
            &StepState::PopEnv => formatter.debug_tuple("PopEnv")
                                           .finish(),
            &StepState::Return => formatter.debug_tuple("Return")
                                           .finish(),
            &StepState::Complete(ref v) => formatter.debug_tuple("Complete")
                                                    .field(v)
                                                    .finish(),
            &StepState::ArgCollectingLambda { ref procedure, ref evaluated, .. } =>
                formatter.debug_struct("ArgCollectingLambda")
                         .field("procedure", procedure)
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
                         .finish(),
        }
    }
}
