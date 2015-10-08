use std::rc::Rc;
use ::{Value, AresError, AresResult};
use super::util::expect_arity;

macro_rules! gen_fold {
    ($args: expr, $default: expr, $var: path, $op: expr, $extr: expr) => {
        {
        let mut cur = $default;
        for a in $args {
            if let &$var(name) = a {
                ($op)(&mut cur, name);
            } else {
                return Err(AresError::UnexpectedType {
                    value: a.clone(),
                    expected: stringify!($var).to_string()
                });
            }
        }
        let res: AresResult<_> = $extr(cur);
        let res = try!(res);
        Ok($var(res))
        }
    };
    ($args: expr, $default: expr, $var: path, $op: expr) => {
        gen_fold!($args, $default, $var, $op, |a| Ok(a))
    }
}

pub fn add_ints(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc = (*acc).wrapping_add(v))
}

pub fn add_floats(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc += v)
}

pub fn sub_ints(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, None, Value::Int, |acc: &mut Option<(bool, i64)>, v: i64| {
        if let &mut Some((ref mut first, ref mut acc)) = acc {
            if *first {
                *acc = -*acc;
            }
            *acc = (*acc).wrapping_sub(v)
        } else {
            *acc = Some((true, -v))
        }
    }, |a: Option<(bool, i64)>| {
        match a {
            Some((_, r)) => Ok(r),
            None => Err(AresError::UnexpectedArity {
                found: 0,
                expected: "at least 1".into()
            })
        }
    })
}

pub fn sub_floats(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, None, Value::Float, |acc: &mut Option<(bool, f64)>, v: f64| {
        if let &mut Some((ref mut first, ref mut acc)) = acc {
            if *first {
                *acc = -*acc;
            }
            *acc -= v
        } else {
            *acc = Some((true, -v))
        }
    }, |a: Option<(bool, f64)>| {
        match a {
            Some((_, r)) => Ok(r),
            None => Err(AresError::UnexpectedArity {
                found: 0,
                expected: "at least 1".into()
            })
        }
    })
}

pub fn mul_ints(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, 1i64, Value::Int, |acc: &mut i64, v: i64| *acc = (*acc).wrapping_mul(v))
}

pub fn mul_floats(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, 1.0f64, Value::Float, |acc: &mut f64, v: f64| *acc *= v)
}

pub fn div_ints(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, None, Value::Int, |acc: &mut Option<i64>, v: i64| {
        if let &mut Some(ref mut acc) = acc {
            *acc = (*acc).wrapping_div(v)
        } else {
            *acc = Some(v)
        }
    }, |a: Option<i64>| {
        match a {
            Some(r) => Ok(r),
            None => Err(AresError::UnexpectedArity {
                found: 0,
                expected: "at least 1".into()
            })
        }
    })
}

pub fn div_floats(args: &[Value]) -> AresResult<Value> {
    gen_fold!(args, 1.0f64, Value::Float, |acc: &mut f64, v: f64| *acc /= v)
}

pub fn modulo(args: &[Value]) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    match (&args[0], &args[1]) {
        (&Value::Int(n1), &Value::Int(n2)) => Ok(Value::Int(n1 % n2)),
        (other, &Value::Int(_)) => Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Int".to_owned()
        }),
        (_, other) => Err(AresError::UnexpectedType {
            value: other.clone(),
            expected: "Int".to_owned()
        })
    }
}


// TODO: move this to a new strings module
pub fn concat(args: &[Value]) -> AresResult<Value> {
    let mut buffer = String::new();
    for v in args {
        if let &Value::String(ref s) = v {
            buffer.push_str(&s[..])
        } else {
            return Err(AresError::UnexpectedType {
                value: v.clone(),
                expected: "Value::String".into()
            })
        }
    }

    Ok(Value::String(Rc::new(buffer)))
}

