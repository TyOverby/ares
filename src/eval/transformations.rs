use super::StepState;

use ::{Value, AresError, AresResult};

use super::procedure::Procedure;
use super::context::{LoadedContext, State};

use super::{apply_lambda, apply_function};

/// Transforms a pre-evaluated callable into a
/// collecting-args lambda.
pub fn from_pre_evaluated<S: ?Sized>(mut unevaluated: Vec<Value>, function: Value, ctx: &mut LoadedContext<S>)
-> AresResult<()>
where S: State {
    // Check to make sure that we actually got something that is callable
    let procedure = match function {
        Value::Lambda(procedure, _) => procedure,
        Value::ForeignFn(func) => {
            let apply_result = try!(apply_function(func, unevaluated, ctx));
            ctx.stack.push(StepState::Complete(apply_result));
            return Ok(())
        }
        other => return Err(AresError::UnexecutableValue(other.clone())),
    };

    // we will be wanting the "next" element very often,
    // so reverse this right now and call `pop` to get the next one
    // in the correct order.
    unevaluated.reverse();

    if let Some(first) = unevaluated.pop() {
        // If we have at least one argument to pass in, start evaluating
        // that one, and build up an ArgCollectingLambda in order to
        // collect all the evaluated arguments.
        ctx.stack.push(StepState::ArgCollectingLambda {
            procedure: procedure,
            evaluated: vec![],
            unevaluated: unevaluated,
        });
        ctx.stack.push(StepState::EvalThis(first, false));
    } else {
        //  If there's no arguments, we can just apply the
        //  lambda right now.
        try!(apply_lambda(procedure, vec![], ctx));
    }
    Ok(())
}

/// This is called when an arg_collecting_lambda gets one of its
/// arguments evaluated.
pub fn from_arg_collecting_lambda<S: ?Sized>(
        procedure: Procedure,
        mut unevaluated: Vec<Value>,
        mut evaluated: Vec<Value>,
        completed: Value,
        ctx: &mut LoadedContext<S>) -> AresResult<()>
where S: State {
    // Push the completed arg-value back on the list of
    // evaluated args.
    evaluated.push(completed);

    if let Some(next) = unevaluated.pop() {
        // If there's another argument to evaluate, do that
        ctx.stack.push(StepState::ArgCollectingLambda {
            procedure: procedure,
            evaluated: evaluated,
            unevaluated: unevaluated,
        });
        ctx.stack.push(StepState::EvalThis(next, false));
    } else {
        // Otherwise call the lambda right now!
        try!(apply_lambda(procedure, evaluated, ctx));
    }
    Ok(())
}

/// This is called when an evaluating-lambda has finished evaluating
/// one of the bodies of a lambda and has produced a result.
pub fn from_evaluating_lambda<S: ?Sized>(
    mut bodies: Vec<Value>,
    name: Option<String>,
    past_body_result: Value,
    ctx: &mut LoadedContext<S>) -> AresResult<()>
where S: State {
    if let Some(next_body) = bodies.pop() {
        // If this isn't the last body in the lambda, start evaluating the
        // next one.
        let body_eval = StepState::EvalThis(next_body, false);
        let watching_state = StepState::EvaluatingLambda {
            name: name,
            bodies: bodies,
        };
        ctx.stack.push(watching_state);
        ctx.stack.push(body_eval);
    } else {
        // If this was the last body in the lambda, then
        // this is the result of that lambda being executed.
        ctx.stack.push(StepState::Complete(past_body_result));
    }
    Ok(())
}
