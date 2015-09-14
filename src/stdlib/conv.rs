use std::collections::HashSet;
use std::rc::Rc;

use ::{Value, AresResult, AresError};

fn to_int(value: Value) -> AresResult<Value> {
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

fn to_float(value: Value) -> Value {
     match value {
         Value::Int(i) => Value::Float(i as f64),
         Value::Float(f) => Value::Float(f),
         Value::String(s) => Value::Float(s.parse().unwrap()),
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Float".to_string()
         })
     }
}

/*
fn to_string(value: Value) -> Value {
    match value {
        Value::Int(i) => Value::String(Rc::new(format!("{}", i))),
        Value::Float(f) => Value::String(Rc::new(format!("{}", f))),
        Value::String(s) => Value::String(s),
        Value::Bool(b) => Value::String(Rc::new(format!("{}", b))),
        l@Value::List(_) => {
            fn build_buf(cur: Value, buf: &mut String, seen: &mut HashSet<&Value>) {
                match cur {
                    Value::List(list) => {

                    }
                    other => {
                        buf.push_str(&to_string(other))
                    }
                }
            }

            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(l, &mut inner, &mut seen);
            //Value::String(Rc::new(format!()))
            inner
        }
        Value::ForeignFn(_) => panic!("can not convert a foreign function to an int"),
        Value::Lambda(_) => panic!("can not convert a lambda to an int"),
        Value::Ident(_) => unreachable!(),
    }
}*/
