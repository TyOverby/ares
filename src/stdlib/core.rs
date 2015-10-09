use std::rc::Rc;
use std::collections::HashMap;
use ::{Value, Procedure, AresResult, AresError, ParamBinding, LoadedContext, State, Environment};
use super::util::expect_arity;
use ::intern::Symbol;

pub fn equals(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let first = &args[0];

    for next in args.iter().skip(1) {
        if *next != *first {
            return Ok(Value::Bool(false))
        }
    }

    Ok(Value::Bool(true))
}

pub fn lett<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >=2, "at least 2"));
    let bindings = &args[0];
    let bodies = &args[1 ..];

    let bindings = match bindings {
        &Value::List(ref inner) => inner,
        other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "List".into()
        })
    };

    try!(expect_arity(&**bindings, |l| l % 2 == 0, "an even number"));

    let mut new_env = Environment::new_with_data(ctx.env().clone(), HashMap::new());
    for pair in bindings.chunks(2) {
        let (name, value) = (&pair[0], &pair[1]);

        let name = match name {
            &Value::Symbol(s) => s,
            other => return Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "Symbol".into()
            })
        };

        let evaluated = ctx.with_other_env(&mut new_env, move |new_ctx| {
            new_ctx.eval(value)
        });
        let evaluated = try!(evaluated);

        new_env.borrow_mut().insert_here(name, evaluated.clone());
    }

    let mut last = None;
    try!(ctx.with_other_env(&mut new_env, |new_ctx| -> AresResult<()> {
        for body in bodies {
            last = Some(try!(new_ctx.eval(body)));
        }
        Ok(())
    }));

    Ok(last.unwrap())
}

pub fn eval<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    ctx.eval(&args[0])
}

pub fn apply<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let func = &args[0];
    let arguments = &args[1];
    match arguments {
        &Value::List(ref lst) => ctx.call(func , lst),
        other => Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "List".into()
        })
    }
}

pub fn lambda<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let param_names = match &args[0] {
        &Value::List(ref v) => {
            let r: AresResult<Vec<Symbol>> = v.iter().map(|n| {
                match n {
                    &Value::Symbol(s) => Ok(s),
                    &ref other => Err(AresError::UnexpectedType {
                        value: other.clone(),
                        expected: "Symbol".into()
                    })
                }
            }).collect();
            ParamBinding::ParamList(try!(r))
        }
        &Value::Symbol(s) => {
            ParamBinding::SingleIdent(s)
        }
        x => {
            return Err(AresError::UnexpectedArgsList(x.clone()));
        }
    };

    let bodies:Vec<_> = args.iter().skip(1).cloned().collect();

    Ok(Value::Lambda(
            Procedure::new(
                None,
                Rc::new(bodies),
                param_names,
                ctx.env().clone())))
}

pub fn define<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name = match &args[0] {
        &Value::Symbol(s) => s,
        &ref other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Symbol".into()
        }),
    };

    if ctx.env().borrow().is_defined_at_this_level(name) {
        return Err(AresError::AlreadyDefined(ctx.interner().lookup_or_unknown(name).into()))
    }

    let value = &args[1];
    let result = try!(ctx.eval(value));

    ctx.env().borrow_mut().insert_here(name, result.clone());
    Ok(result)
}

pub fn set<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name = match &args[0] {
        &Value::Symbol(s) => s,
        &ref v => return Err(AresError::UnexpectedType {
            value: v.clone(),
            expected: "Symbol".into()
        }),
    };

    let value = &args[1];

    if !ctx.env().borrow().is_defined(name) {
        return Err(AresError::UndefinedName(ctx.interner().lookup_or_unknown(name).into()))
    }

    let result = try!(ctx.eval(value));
    ctx.env().borrow_mut().with_value_mut(name, |v| *v = result.clone());
    Ok(result)
}

pub fn quote<S: State + ?Sized>(args: &[Value], _ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    Ok(args[0].clone())
}

pub fn cond<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 3, "exactly 3"));
    let (cond, true_branch, false_branch) = (&args[0], &args[1], &args[2]);
    match try!(ctx.eval(cond)) {
        Value::Bool(true) => ctx.eval(true_branch),
        Value::Bool(false) => ctx.eval(false_branch),
        other => Err(AresError::UnexpectedType {
            value: other,
            expected: "Bool".into()
        })
    }
}

pub fn gensym<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 0 || l == 1, "either 0 or 1"));
    let symbol = if args.len() == 0 {
        ctx.interner_mut().gen_sym_prefix("s")
    } else {
        match &args[0] {
            &Value::String(ref s) => ctx.interner_mut().gen_sym_prefix(&s[..]),
            other => return Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "String".into()
            })
        }
    };
    Ok(Value::Symbol(symbol))
}
