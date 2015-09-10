use super::Value;

macro_rules! gen_fold {
    ($args: expr, $default: expr, $var: path, $op: expr) => {
        {
        let mut cur = $default;
        for a in $args {
            if let $var(name) = a {
                ($op)(&mut cur, name);
            } else {
                panic!("bad type for fold");
            }
        }
        $var(cur)
        }
    }
}

pub fn add_ints(args: Vec<Value>) -> Value {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc += v)
}

pub fn add_floats(args: Vec<Value>) -> Value {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc += v)
}

pub fn sub_ints(args: Vec<Value>) -> Value {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc -= v)
}

pub fn sub_floats(args: Vec<Value>) -> Value {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc -= v)
}

pub fn mul_ints(args: Vec<Value>) -> Value {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc *= v)
}

pub fn mul_floats(args: Vec<Value>) -> Value {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc *= v)
}

pub fn div_ints(args: Vec<Value>) -> Value {
    gen_fold!(args, 0i64, Value::Int, |acc: &mut i64, v: i64| *acc /= v)
}

pub fn div_floats(args: Vec<Value>) -> Value {
    gen_fold!(args, 0.0f64, Value::Float, |acc: &mut f64, v: f64| *acc /= v)
}

pub fn concat(args: Vec<Value>) -> Value {
    let mut buffer = String::new();
    for v in args {
        if let Value::String(s) = v {
            buffer.push_str(&s)
        } else {
            panic!("concat can't concatenate non-strings");
        }
    }

    Value::String(buffer)
}
