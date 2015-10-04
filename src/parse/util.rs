use super::super::Value;
use super::super::Value::*;

pub fn immediate_value(v: &Value) -> bool {
    match v {
        &Map(ref m) => m.iter().all(|(k, v)| immediate_value(k) && immediate_value(v)),
        &List(ref vec) => vec.len() == 2 && vec[0] == Value::ident("quote"),
        &Ident(_) => false,
        _ => true
    }
}

pub fn unquote(v: Value) -> Value {
    match v {
        List(vec) => vec[1].clone(),
        v => v
    }
}

pub fn can_be_hash_key(v: &Value) -> bool {
    match v {
        &Map(..) | &Ident(..) => false,
        &List(ref vec) => vec.len() == 2 && vec[0] == Value::ident("quote") && match &vec[1] {
            &Map(..) | &List(..) => false,
            _ => true
        },
        _ => true
    }
}
