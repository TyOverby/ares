use std::collections::HashSet;
use std::rc::Rc;

use ::Value;

fn to_int(value: Value) -> Value {
     match value {
         Value::Int(i) => Value::Int(i),
         Value::Float(f) => Value::Int(f as i64),
         Value::Bool(b) => Value::Int(if b {1} else {0}),
         Value::String(s) => Value::Int(s.parse().unwrap()),
         Value::List(_) => panic!("can not convert a list to an int"),
         Value::ForeignFn(_) => panic!("can not convert a foreign function to an int"),
         Value::Lambda(_) => panic!("can not convert a lambda to an int"),
         Value::Ident(_) => unreachable!(),
     }
}

fn to_float(value: Value) -> Value {
     match value {
         Value::Int(i) => Value::Float(i as f64),
         Value::Float(f) => Value::Float(f),
         Value::String(s) => Value::Float(s.parse().unwrap()),
         Value::Bool(b) => panic!("can not convert a boolean to a float"),
         Value::List(_) => panic!("can not convert a list to a float"),
         Value::ForeignFn(_) => panic!("can not convert a foreign function to a float"),
         Value::Lambda(_) => panic!("can not convert a lambda to a float"),
         Value::Ident(_) => panic!("can not convert an identifier to a float")
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
