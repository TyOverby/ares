use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use {Value, AresResult, LoadedContext, State, user_fn, free_fn, Environment};
use util::prompt;
use intern::Symbol;
use super::util::expect_arity;

pub fn debugger<S: State + ?Sized>(args: &[Value],
                                   ctx: &mut LoadedContext<S>)
                                   -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 0, "exactly 0"));

    let result = Rc::new(RefCell::new(None));
    let result_writer = result.clone();

    let debugger_close = move |values: &[Value]| -> AresResult<Value> {
        try!(expect_arity(values, |l| l == 1, "exactly 1"));
        *result_writer.borrow_mut() = Some(values[0].clone());
        Ok(values[0].clone())
    };

    let debugger_env = move |values: &[Value], ctx: &mut LoadedContext<S>| -> AresResult<Value> {
        try!(expect_arity(values, |l| l == 0, "exactly 0"));
        let mut list: Vec<(Symbol, (u32, Value))> = ctx.env()
                                                       .borrow()
                                                       .all_defined()
                                                       .into_iter()
                                                       .collect();
        // Invert the sort
        list.sort_by(|a, b| (b.1).0.cmp(&(a.1).0));

        let mut last_level = (list[0].1).0;
        for (name, (level, value)) in list {
            if level < last_level {
                println!("");
                last_level = level;
            }

            println!("{}: {:?}", ctx.interner().lookup_or_anon(name), value);
        }
        Ok(false.into())
    };

    let debugger_close: Value = Value::ForeignFn(free_fn::<S, _, _>("debugger-close",
                                                                    debugger_close)
                                                     .erase());
    let debugger_env: Value = Value::ForeignFn(user_fn::<S, _, _>("debugger-env", debugger_env)
                                                   .erase());
    let mut mapping = HashMap::new();
    mapping.insert(ctx.interner_mut().intern("debugger-close"), debugger_close);
    mapping.insert(ctx.interner_mut().intern("debugger-env"), debugger_env);
    let new_env = Environment::new_with_data(ctx.env().clone(), mapping);

    ctx.with_other_env(new_env, |ctx| {
        while result.borrow().is_none() {
            let line = match prompt("debugger> ") {
                Some(line) => line,
                None => break,
            };

            let res = ctx.eval_str(&line);
            if result.borrow().is_none() {
                match res {
                    Ok(v) => println!("{:?}", v),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    });

    let mut result = result.borrow_mut();
    Ok(result.take().unwrap())
}
