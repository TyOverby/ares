use ::{Value, AresResult, AresError, LoadedContext, State};

pub fn and<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    for value in args {
        match try!(ctx.eval(value)) {
            Value::Bool(true) => { }
            Value::Bool(false) => return Ok(Value::Bool(false)),
            other => return Err(AresError::UnexpectedType {
                value: other,
                expected: "Bool".into()
            })
        }
    }
    Ok(Value::Bool(true))
}

pub fn or<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    for value in args {
        match try!(ctx.eval(value)) {
            Value::Bool(true) => return Ok(Value::Bool(true)),
            Value::Bool(false) => {},
            other => return Err(AresError::UnexpectedType {
                value: other,
                expected: "Bool".into()
            })
        }
    }
    Ok(Value::Bool(false))
}

pub fn xor<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    let mut found_true = false;
    let mut found_false = false;
    for value in args {
        match try!(ctx.eval(value)) {
            Value::Bool(true) => {
                found_true = true;
                if found_true && found_false {
                    return Ok(Value::Bool(true))
                }
            }
            Value::Bool(false) => {
                found_false = true;
                if found_true && found_false {
                    return Ok(Value::Bool(true))
                }
            }
            other => return Err(AresError::UnexpectedType {
                value: other,
                expected: "Bool".into()
            })
        }
    }
    Ok(Value::Bool(false))
}
