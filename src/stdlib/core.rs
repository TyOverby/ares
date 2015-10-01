use std::rc::Rc;
use ::{Value, Procedure, AresResult, AresError, ParamBinding, LoadedContext};
use super::util::expect_arity;

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

pub fn lambda(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
    try!(expect_arity(args, |l| l >= 2, "at least 2"));
    let param_names = match &args[0] {
        &Value::List(ref v) => {
            let r: AresResult<Vec<String>> = v.iter().map(|n| {
                match n {
                    &Value::Ident(ref s) => Ok((&**s).clone()),
                    &ref other => Err(AresError::UnexpectedType {
                        value: other.clone(),
                        expected: "Ident".into()
                    })
                }
            }).collect();
            ParamBinding::ParamList(try!(r))
        }
        &Value::Ident(ref v) => {
            ParamBinding::SingleIdent((**v).clone())
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

pub fn define(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name: String = match &args[0] {
        &Value::Ident(ref s) => (**s).clone(),
        &ref other => return Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Ident".into()
        }),
    };

    if ctx.env().borrow().is_defined_at_this_level(&name) {
        return Err(AresError::AlreadyDefined(name.into()))
    }

    let value = &args[1];
    let result = try!(ctx.eval(value));

    ctx.env().borrow_mut().insert_here(name, result.clone());
    Ok(result)
}

pub fn set(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let name = match &args[0] {
        &Value::Ident(ref s) => (**s).clone(),
        &ref v => return Err(AresError::UnexpectedType {
            value: v.clone(),
            expected: "Ident".into()
        }),
    };

    let value = &args[1];

    if !ctx.env().borrow().is_defined(&name) {
        return Err(AresError::UndefinedName(name.into()))
    }

    let result = try!(ctx.eval(value));
    ctx.env().borrow_mut().with_value_mut(&name, |v| *v = result.clone());
    Ok(result)
}

pub fn quote(args: &[Value], _ctx: &mut LoadedContext) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    Ok(args[0].clone())
}

pub fn cond(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
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
