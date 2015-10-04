use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use ::{Value, AresResult, LoadedContext, State, free_fn, Environment};
use ::util::prompt;
use super::util::expect_arity;

pub fn debugger<S: State + ?Sized>(_args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    let result = Rc::new(RefCell::new(None));
    let result_writer = result.clone();

    let debugger_close = move |values: &[Value]| -> AresResult<Value> {
        try!(expect_arity(values, |l| l == 1, "exactly 1"));
        *result_writer.borrow_mut() = Some(values[0].clone());
        Ok(values[0].clone())
    };

    let debugger_env = move |values: &[Value]| -> AresResult<Value> {
        try!(expect_arity(values, |l| l == 0, "exactly 0"));
        println!("debugger-env");
        // TODO: print the environment
        Ok(false.into())
    };

    let debugger_close: Value = Value::ForeignFn(free_fn::<S, _, _>("debugger-close", debugger_close).erase());
    let debugger_env: Value = Value::ForeignFn(free_fn::<S, _, _>("debugger-env", debugger_env).erase());
    let mut mapping = HashMap::new();
    mapping.insert("debugger-close".to_owned(), debugger_close);
    mapping.insert("debugger-env".to_owned(), debugger_env);
    let mut new_env = Environment::new_with_data(ctx.env().clone(), mapping);

    ctx.with_other_env(&mut new_env, |ctx| {
        while result.borrow().is_none() {
            let line = match prompt("debugger> ") {
                Some(line) => line,
                None => break
            };

            let res = ctx.eval_str(&line);
            if result.borrow().is_none() {
                match res {
                    Ok(v)  => println!("{:?}", v),
                    Err(e) => println!("{:?}", e)
                }
            }
        }
    });

    let mut result = result.borrow_mut();
    Ok(result.take().unwrap())
}

