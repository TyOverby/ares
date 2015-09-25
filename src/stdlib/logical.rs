use ::{Value, AresResult, Env, AresError};

pub fn and(args: &mut Iterator<Item=&Value>,
           env: &Env,
           eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    for value in args {
        match try!(eval(value, env)) {
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

pub fn or(args: &mut Iterator<Item=&Value>,
           env: &Env,
           eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    for value in args {
        match try!(eval(value, env)) {
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

pub fn xor(args: &mut Iterator<Item=&Value>,
           env: &Env,
           eval: &Fn(&Value, &Env) -> AresResult<Value>) -> AresResult<Value> {
    let mut found_true = false;
    let mut found_false = false;
    for value in args {
        match try!(eval(value, env)) {
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
