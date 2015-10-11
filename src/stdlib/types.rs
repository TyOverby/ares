use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;

use ::{Value, AresResult, AresError, rc_to_usize, State, LoadedContext};
use ::intern::SymbolIntern;
use super::util::expect_arity;

macro_rules! gen_is_type {
    ($name: ident, $p: ident) => {
        pub fn $name(values: &[Value]) -> AresResult<Value> {
            for item in values {
                if let &Value::$p(..) = item {
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
gen_is_type!(is_symbol, Symbol);
gen_is_type!(is_lambda, Lambda);
gen_is_type!(is_foreign_fn, ForeignFn);

pub fn is_executable(values: &[Value]) -> AresResult<Value> {
    for item in values {
        match item {
            &Value::Lambda(_, _) => {},
            &Value::ForeignFn(_) => {},
            _ => return Ok(false.into())
        }
    }

    Ok(true.into())
}


pub fn to_int(values: &[Value]) -> AresResult<Value> {
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

pub fn to_float(values: &[Value]) -> AresResult<Value> {
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

pub fn to_bool(values: &[Value]) -> AresResult<Value> {
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

pub fn to_string<S: State + ?Sized>(values: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(values, |l| l == 1, "exactly 1"));
    let first = values.first().unwrap();
    let s = to_string_helper(&first, ctx.interner());
    Ok(Value::String(Rc::new(s)))
}

// TODO: move this out of stdlib?  Seems way too useful.
pub fn to_string_helper(value: &Value, interner: &SymbolIntern) -> String {
    match value {
        &Value::Int(i) => format!("{}", i),
        &Value::Float(f) => format!("{}", f),
        &Value::String(ref s) => (&**s).clone(),
        &Value::Bool(b) => format!("{}", b),
        &Value::ForeignFn(ref ff) => format!("<#{}>", ff.name),
        &Value::Lambda(ref l, _) =>
            format!("<@{}>", l.name.as_ref().map(|s| &s[..]).unwrap_or("anonymous")),
        &Value::UserData(ref u) => format!("UserData@{}", rc_to_usize(u)),
        &Value::Symbol(s) => format!("'{}", interner.lookup_or_anon(s)),

        &ref l@Value::List(_) | &ref l@Value::Map(_) => {
            fn format_singles(vec: &Rc<Vec<Value>>, buf: &mut String, seen: &mut HashSet<usize>, interner: &SymbolIntern) {
                let ptr = rc_to_usize(vec);
                if seen.contains(&ptr) {
                    buf.push_str("...")
                } else {
                    seen.insert(ptr);
                    buf.push_str("[");
                    for v in vec.iter() {
                        build_buf(v, buf, seen, interner);
                        buf.push_str(", ");
                    }
                    // remove trailing comma ans space
                    buf.pop();
                    buf.pop();
                    buf.push_str("]");
                }
            }
            fn format_pairs(m: &Rc<HashMap<Value, Value>>, buf: &mut String, seen: &mut HashSet<usize>, interner: &SymbolIntern) {
                let ptr = rc_to_usize(m);
                if seen.contains(&ptr) {
                    buf.push_str("...")
                } else {
                    seen.insert(ptr);
                    buf.push_str("{");
                    for (k, v) in m.iter() {
                        build_buf(k, buf, seen, interner);
                        buf.push_str(", ");
                        build_buf(v, buf, seen, interner);
                    }
                    buf.push_str("}")
                }
            }
            fn build_buf(cur: &Value, buf: &mut String, seen: &mut HashSet<usize>, interner: &SymbolIntern) {
                match cur {
                    &Value::List(ref v) => format_singles(v, buf, seen, interner),
                    &Value::Map(ref m) => format_pairs(m, buf, seen, interner),
                    other => buf.push_str(&to_string_helper(&other, interner))
                }
            }
            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(&l, &mut inner, &mut seen, interner);
            inner
        }
    }
}
