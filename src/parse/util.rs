use super::super::Value;
use super::super::Value::*;
use ::intern::SymbolIntern;

pub fn immediate_value(v: &Value, interner: &mut SymbolIntern) -> bool {
    match v {
        &Map(ref m) => m.iter().all(|(k, v)| immediate_value(k, interner) &&
                                             immediate_value(v, interner)),
        &List(ref vec) => vec.len() == 2 && vec[0] == Value::Symbol(interner.intern("quote")),
        &Symbol(_) => false,
        _ => true
    }
}

pub fn unquote(v: Value) -> Value {
    match v {
        List(vec) => vec[1].clone(),
        v => v
    }
}

pub fn can_be_hash_key(v: &Value, interner: &mut SymbolIntern) -> bool {
    match v {
        &Map(..) | &Symbol(..) => false,
        &List(ref vec) => vec.len() == 2 && vec[0] == Value::Symbol(interner.intern("quote")) && match &vec[1] {
            &Map(..) | &List(..) => false,
            _ => true
        },
        _ => true
    }
}
