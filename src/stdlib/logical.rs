use ::{Value, AresResult, Env, AresError, LoadedContext};

pub fn and<S>(args: &[Value<S>], ctx: &mut LoadedContext<S>) -> AresResult<Value<S>, S> {
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

pub fn or<S>(args: &[Value<S>], ctx: &mut LoadedContext<S>) -> AresResult<Value<S>, S> {
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

pub fn xor<S>(args: &[Value<S>], ctx: &mut LoadedContext<S>) -> AresResult<Value<S>, S> {
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
