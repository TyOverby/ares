use super::super::Value;
use super::super::Value::*;

pub fn immediate_value(v: &Value) -> bool {
    match v {
        &Map(ref m) => m.iter().all(|(k, v)| immediate_value(k) && immediate_value(v)),
        &List(ref vec) => vec.len() >= 1 && vec[0] == Value::new_ident("quote"),
        &Ident(_) => false,
        _ => true
    }
}

pub fn can_be_hash_key(v: &Value) -> bool {
    match v {
        &Map(..) | &Ident(..) => false,
        &List(ref vec) => vec.len() == 2 && vec[0] == Value::new_ident("quote") && match &vec[1] {
            &Map(..) | &List(..) => false,
            _ => true
        },
        _ => true
    }
}
