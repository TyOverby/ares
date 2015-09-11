use std::rc::Rc;

use super::Value;

macro_rules! gen_fold {
    ($args: expr, $default: expr, $var: path, $op: expr, $extr: expr) => {
        {
        let mut cur = $default;
        for a in $args {
            if let $var(name) = a {
                ($op)(&mut cur, name);
            } else {
                panic!("bad type for fold");
            }
        }
        $var($extr(cur))
        }
    };
    ($args: expr, $default: expr, $var: path, $op: expr) => {
        gen_fold!($args, $default, $var, $op, |a| a)
    }
}

pub fn add_ints(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc += v)
}

pub fn add_floats(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc += v)
}

pub fn sub_ints(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, None, Value::Int, |acc: &mut Option<(bool, i64)>, v: i64| {
        if let &mut Some((ref mut first, ref mut acc)) = acc {
            if *first {
                *acc = -*acc;
            }
            *acc -= v
        } else {
            *acc = Some((true, -v))
        }
    }, |a: Option<(bool, i64)>| a.expect("subtraction expects at least one value").1)
}

pub fn sub_floats(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, None, Value::Float, |acc: &mut Option<(bool, f64)>, v: f64| {
        if let &mut Some((ref mut first, ref mut acc)) = acc {
            if *first {
                *acc = -*acc;
            }
            *acc -= v
        } else {
            *acc = Some((true, -v))
        }
    }, |a: Option<(bool, f64)>| a.expect("subtraction expects at least one value").1)
}

pub fn mul_ints(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, 1i64, Value::Int, |acc: &mut i64, v: i64| *acc *= v)
}

pub fn mul_floats(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, 1.0f64, Value::Float, |acc: &mut f64, v: f64| *acc *= v)
}

pub fn div_ints(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, None, Value::Int, |acc: &mut Option<i64>, v: i64| {
        if let &mut Some(ref mut acc) = acc {
            *acc /= v
        } else {
            *acc = Some(v)
        }
    }, |a: Option<i64>| a.expect("subtraction expects at least one value"))
}

pub fn div_floats(args: &mut Iterator<Item=Value>) -> Value {
    gen_fold!(args, 1.0f64, Value::Float, |acc: &mut f64, v: f64| *acc /= v)
}

pub fn concat(args: &mut Iterator<Item=Value>) -> Value {
    let mut buffer = String::new();
    for v in args {
        if let Value::String(s) = v {
            buffer.push_str(&s)
        } else {
            panic!("concat can't concatenate non-strings");
        }
    }

    Value::String(Rc::new(buffer))
}
