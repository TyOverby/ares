use super::super::Value;
use super::super::Value::*;

pub fn immediate_value(v: &Value) -> bool {
    match v {
        &Map(ref m) => m.iter().all(|(k, v)| immediate_value(k) && immediate_value(v)),
        &List(ref vec) => vec.len() >= 1 && vec[0] == "quote".into(),
        _ => true
    }
}
