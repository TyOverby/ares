use std::rc::Rc;
use std::collections::HashMap;
use {Value, Procedure, AresResult, AresError, ParamBinding, LoadedContext, State, Environment};
use super::util::expect_arity;
use intern::Symbol;

pub fn equals(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let first = &args[0];

    for next in args.iter().skip(1) {
        if *next != *first {
            return Ok(Value::Bool(false));
        }
    }

    Ok(Value::Bool(true))
}

pub fn lett<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let bindings = &args[0];
    let bodies = &args[1..];

    let bindings = match bindings {
        &Value::List(ref inner) => inner,
        other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "List".into(),
        }),
    };

    try!(expect_arity(&**bindings, |l| l % 2 == 0, "an even number"));

    let mut new_env = Environment::new_with_data(ctx.env().clone(), HashMap::new());
    for pair in bindings.chunks(2) {
        let (name, value) = (&pair[0], &pair[1]);

        let name = match name {
            &Value::Symbol(s) => s,
            other => return Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "Symbol".into(),
            }),
        };

        let evaluated = ctx.with_other_env(&mut new_env, move |new_ctx| new_ctx.eval(value));
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
        &Value::List(ref lst) => ctx.call(func, lst),
        other => Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "List".into(),
        }),
    }
}

pub fn lambda<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let dot = ctx.interner_mut().intern(".");
    let param_names = match &args[0] {
        &Value::List(ref v) => {
            let mut params = vec![];
            let mut rest = None;
            let mut seen_dot = false;
            try!(v.iter()
                  .map(|n| {
                      match n {
                          &Value::Symbol(s) if s == dot => {
                              if seen_dot {
                                  Err(AresError::UnexpectedArgsList(args[0].clone()))
                              } else {
                                  seen_dot = true;
                                  Ok(())
                              }
                          }
                          &Value::Symbol(s) => {
                              if seen_dot {
                                  match rest {
                                      None => {
                                          rest = Some(s);
                                          Ok(())
                                      }
                                      Some(_) =>
                                          Err(AresError::UnexpectedArgsList(args[0].clone())),
                                  }
                              } else {
                                  params.push(s);
                                  Ok(())
                              }
                          }
                          &ref other => Err(AresError::UnexpectedType {
                              value: other.clone(),
                              expected: "Symbol".into(),
                          }),
                      }
                  })
                  .collect::<AresResult<Vec<()>>>());
            ParamBinding {
                params: params,
                rest: rest,
            }
        }
        &Value::Symbol(s) => {
            ParamBinding {
                params: vec![],
                rest: Some(s),
            }
        }
        x => {
            return Err(AresError::UnexpectedArgsList(x.clone()));
        }
    };

    let bodies: Vec<_> = args.iter().skip(1).cloned().collect();

    Ok(Value::Lambda(Procedure::new(None, Rc::new(bodies), param_names, ctx.env().clone()),
                     false))
}

fn define_helper<S: State + ?Sized>(args: &[Value],
                                    ctx: &mut LoadedContext<S>)
                                    -> AresResult<(Symbol, Value)> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name = match &args[0] {
        &Value::Symbol(s) => s,
        &ref other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Symbol".into(),
        }),
    };

    if ctx.env().borrow().is_defined_at_this_level(name) {
        return Err(AresError::AlreadyDefined(ctx.interner().lookup_or_anon(name)));
    }

    let value = &args[1];
    let result = try!(ctx.eval(value));
    Ok((name, result))
}


pub fn define<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    let (name, value) = try!(define_helper(args, ctx));
    ctx.env().borrow_mut().insert_here(name, value.clone());
    Ok(value)
}

pub fn define_macro<S: State + ?Sized>(args: &[Value],
                                       ctx: &mut LoadedContext<S>)
                                       -> AresResult<Value> {
    let (name, value) = try!(define_helper(args, ctx));
    let procedure = match value {
        Value::Lambda(p, _) => p.clone(),
        other => {
            return Err(AresError::UnexpectedType {
                value: other,
                expected: "Lambda".into(),
            });
        }
    };
    let mac = Value::Lambda(procedure, true);
    ctx.env().borrow_mut().insert_here(name, mac.clone());
    Ok(mac)
}

pub fn walk<F>(value: &Value, f: &mut F) -> AresResult<Value>
    where F: FnMut(&Value) -> AresResult<(Value, bool)>
{
    let (v, recurse) = try!(f(value));
    if recurse {
        match v {
            Value::List(v) => {
                let result = try!(v.iter()
                                   .map(|value| Ok(try!(walk(value, f))))
                                   .collect::<AresResult<Vec<Value>>>());
                Ok(Value::list(result))
            }
            Value::Map(m) => {
                let mut result = HashMap::with_capacity(m.len());
                for (k, v) in m.iter() {
                    let new_k = try!(walk(k, f));
                    let new_v = try!(walk(v, f));
                    result.insert(new_k, new_v);
                }
                Ok(Value::Map(Rc::new(result)))
            }
            v => Ok(v),
        }
    } else {
        Ok(v)
    }
}

pub fn macroexpand<S: State + ?Sized>(args: &[Value],
                                      ctx: &mut LoadedContext<S>)
                                      -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    let quote = ctx.interner_mut().intern("quote");  // this should really be handled better...
    let mut walk_f = |value: &Value| {
        match value {
            &Value::List(ref lst) => {
                if lst.len() == 0 {
                    return Ok((Value::List(lst.clone()), false));
                }
                match &lst[0] {
                    &Value::Symbol(s) if s == quote => Ok((value.clone(), false)),
                    &Value::Symbol(s) => {
                        let v = ctx.env().borrow().get(s);
                        match v {
                            Some(v@Value::Lambda(_, true)) => {
                                let macro_out = try!(ctx.call(&v, &lst[1..lst.len()]));
                                let finished = try!(macroexpand(&[macro_out], ctx));
                                Ok((finished, true))
                            }
                            _ => Ok((value.clone(), true)),
                        }
                    }
                    _ => Ok((value.clone(), true)),
                }
            }
            other => Ok((other.clone(), false)),
        }
    };
    walk(&args[0], &mut walk_f)
}

pub fn set<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name = match &args[0] {
        &Value::Symbol(s) => s,
        &ref v => return Err(AresError::UnexpectedType {
            value: v.clone(),
            expected: "Symbol".into(),
        }),
    };

    let value = &args[1];

    if !ctx.env().borrow().is_defined(name) {
        return Err(AresError::UndefinedName(ctx.interner().lookup_or_anon(name)));
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
            expected: "Bool".into(),
        }),
    }
}

pub fn gensym<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 0 || l == 1, "either 0 or 1"));
    let symbol = if args.len() == 0 {
        ctx.interner_mut().gen_sym()
    } else {
        match &args[0] {
            &Value::String(ref s) => ctx.interner_mut().gen_sym_prefix(&s[..]),
            other => return Err(AresError::UnexpectedType {
                value: other.clone(),
                expected: "String".into(),
            }),
        }
    };
    Ok(Value::Symbol(symbol))
}

pub fn quasiquote<S: State + ?Sized>(args: &[Value],
                                     ctx: &mut LoadedContext<S>)
                                     -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    let unquote = Value::Symbol(ctx.interner_mut().intern("unquote"));
    let unquote_splicing = Value::Symbol(ctx.interner_mut().intern("unquote-splicing"));
    let mut walk_f = |v: &Value| {
        match v {
            &Value::List(ref lst) => {
                if lst.len() >= 1 && lst[0] == unquote_splicing {
                    return Err(AresError::InvalidUnquotation);
                } else if lst.len() == 2 && lst[0] == unquote {
                    return Ok(try!(ctx.eval(&lst[1])));
                }
                let mut new_v = vec![];
                for elem in lst.iter() {
                    match elem {
                        &Value::List(ref inner) => {
                            if inner.len() == 2 && inner[0] == unquote {
                                new_v.push(try!(ctx.eval(&inner[1])));
                            } else if inner.len() == 2 && inner[0] == unquote_splicing {
                                let evald = try!(ctx.eval(&inner[1]));
                                match evald {
                                    Value::List(ref evald) => new_v.extend(evald.iter().cloned()),
                                    _ => return Err(AresError::UnexpectedType {
                                        value: evald,
                                        expected: "list".into(),
                                    }),
                                }
                            } else {
                                let r = try!(quasiquote(&[elem.clone()], ctx));
                                new_v.push(r);
                            }
                        }
                        elem => new_v.push(elem.clone()),
                    }
                }
                Ok(Value::list(new_v))
            }
            _ => Ok(v.clone()),
        }
    };
    walk_f(&args[0])
}

pub fn unquote_error<S: State + ?Sized>(_args: &[Value],
                                        _ctx: &mut LoadedContext<S>)
                                        -> AresResult<Value> {
    Err(AresError::InvalidUnquotation)
}
