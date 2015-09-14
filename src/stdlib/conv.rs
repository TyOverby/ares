use std::collections::HashSet;
use std::rc::Rc;

use ::{Value, AresResult, AresError, rc_to_usize};

pub fn to_int(value: Value) -> AresResult<Value> {
     match value {
         Value::Int(i) => Ok(Value::Int(i)),
         Value::Float(f) => Ok(Value::Int(f as i64)),
         Value::Bool(b) => Ok(Value::Int(if b {1} else {0})),
         Value::String(s) => Ok(Value::Int(s.parse().unwrap())),
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Int".to_string()
         })
     }
}

pub fn to_float(value: Value) -> AresResult<Value> {
     match value {
         Value::Int(i) => Ok(Value::Float(i as f64)),
         Value::Float(f) => Ok(Value::Float(f)),
         Value::String(s) => Ok(Value::Float(s.parse().unwrap())),
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Float".to_string()
         })
     }
}

pub fn to_string(value: Value) -> Value {
    Value::String(Rc::new(as_str(&value)))
}

fn as_str(value: &Value) -> String {
    match value {
        &Value::Int(i) => format!("{}", i),
        &Value::Float(f) => format!("{}", f),
        &Value::String(ref s) => (&**s).clone(),
        &Value::Bool(b) => format!("{}", b),
        & ref l@Value::List(_) => {
            fn build_buf(cur: &Value, buf: &mut String, seen: &mut HashSet<usize>) {
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
                        buf.push_str(&as_str(&other))
                    }
                }
            }

            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(&l, &mut inner, &mut seen);
            inner
        }
        &Value::ForeignFn(_) => panic!("can not convert a foreign function to an int"),
        &Value::Lambda(_) => panic!("can not convert a lambda to an int"),
        &Value::Ident(_) => unreachable!(),
    }
}
