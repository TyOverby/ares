use ::{Value, AresResult, AresError};
use super::util::expect_arity;

pub fn some(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    let first = args[0].clone();
    Ok(Value::Option(Some(Box::new(first))))
}

pub fn none(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 0, "exactly 0"));
    Ok(Value::Option(None))
}

pub fn unwrap(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    match &args[0] {
        &Value::Option(Some(ref res)) => Ok((**res).clone()),
        &Value::Option(None) => Err(AresError::UnwrapNone),
        other => Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Option".into()
        })
    }
}
