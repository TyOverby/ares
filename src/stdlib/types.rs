use std::collections::HashSet;
use std::rc::Rc;

use ::{Value, AresResult, AresError, rc_to_usize};
use super::util::expect_arity;

macro_rules! gen_is_type {
    ($name: ident, $p: ident) => {
        pub fn $name<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
            for item in values {
                if let &Value::$p(_) = item {
                } else {
                    return Ok(false.into())
                }
            }
            Ok(true.into())
        }
    }
}

gen_is_type!(is_int, Int);
gen_is_type!(is_float, Float);
gen_is_type!(is_bool, Bool);
gen_is_type!(is_string, String);
gen_is_type!(is_list, List);
gen_is_type!(is_ident, Ident);
gen_is_type!(is_lambda, Lambda);
gen_is_type!(is_foreign_fn, ForeignFn);

pub fn is_executable<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
    for item in values {
        match item {
            &Value::Lambda(_) => {},
            &Value::ForeignFn(_) => {},
            _ => return Ok(false.into())
        }
    }

    Ok(true.into())
}


pub fn to_int<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
    try!(expect_arity(values, |l| l == 1, "exactly 1"));

    let res = match values.first().unwrap() {
        &Value::Int(i) => Ok(Value::Int(i)),
        &Value::Float(f) => Ok(Value::Int(f as i64)),
        &Value::Bool(b) => Ok(Value::Int(if b {1} else {0})),
        &Value::String(ref s) => Ok(Value::Int(s.parse().unwrap())),
        other => Err(AresError::IllegalConversion {
            value: other.clone(),
            into: "Int".to_string()
        })
    };

    res
}

pub fn to_float<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
    try!(expect_arity(values, |l| l == 1, "exactly 1"));

    let res = match values.first().unwrap() {
        &Value::Int(i) => Ok(Value::Float(i as f64)),
        &Value::Float(f) => Ok(Value::Float(f)),
        &Value::String(ref s) => Ok(Value::Float(s.parse().unwrap())),
        other => Err(AresError::IllegalConversion {
            value: other.clone(),
            into: "Float".to_string()
        })
    };
    res
}

pub fn to_bool<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
    try!(expect_arity(values, |l| l == 1, "exactly 1"));

    let res = match values.first().unwrap() {
        &Value::Int(0) => Ok(Value::Bool(false)),
        &Value::Int(_) => Ok(Value::Bool(true)),
        &Value::Float(0.0) => Ok(Value::Bool(false)),
        // TODO: Float(nan) => Ok(false)?
        &Value::Float(_) => Ok(Value::Bool(true)),
        &Value::Bool(b) => Ok(Value::Bool(b)),
        &Value::String(ref s) => {
            if &**s == "true" {
                Ok(Value::Bool(true))
            } else if &**s == "false" {
                Ok(Value::Bool(false))
            } else {
                Err(AresError::IllegalConversion{
                    value: Value::String(s.clone()),
                    into: "Bool".to_string()
                })
            }
        }
        other => Err(AresError::IllegalConversion {
            value: other.clone(),
            into: "Bool".to_string()
        })
    };

    res
}

pub fn to_string<S>(values: &[Value<S>]) -> AresResult<Value<S>, S> {
    try!(expect_arity(values, |l| l == 1, "exactly 1"));
    let first = values.first().unwrap();
    let s = to_string_helper(&first);
    Ok(Value::String(Rc::new(s)))
}

fn to_string_helper<S>(value: &Value<S>) -> String {
    match value {
        &Value::Int(i) => format!("{}", i),
        &Value::Float(f) => format!("{}", f),
        &Value::String(ref s) => (&**s).clone(),
        &Value::Bool(b) => format!("{}", b),
        &Value::ForeignFn(ref ff) => format!("<#{}>", ff.name),
        &Value::Lambda(ref l) => format!("<@{}>", l.name.as_ref().map(|s| &s[..]).unwrap_or("anonymous")),
        &Value::Ident(ref i) => format!("'{}", i),

        &ref l@Value::List(_) => {
            fn build_buf<S>(cur: &Value<S>, buf: &mut String, seen: &mut HashSet<usize>) {
                match cur {
                    &Value::List(ref l) => {
                        let ptr = rc_to_usize(l);
                        if seen.contains(&ptr) {
                            buf.push_str("...");
                        } else {
                            seen.insert(ptr);
                            buf.push_str("[");
                            for v in l.iter() {
                                build_buf(v, buf, seen);
                                buf.push_str(", ");
                            }
                            // removing trailing comma and space
                            buf.pop();
                            buf.pop();
                            buf.push_str("]");
                        }
                    }
                    other => {
                        buf.push_str(&to_string_helper(&other))
                    }
                }
            }

            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(&l, &mut inner, &mut seen);
            inner
        }
    }
}
