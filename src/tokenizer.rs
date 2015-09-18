// Based on Norvig's lisp interpreter
use std::rc::Rc;

use super::Value;

fn tokenize(s: &str) -> Vec<String> {
    s.replace("(", " ( ").replace(")", " ) ").split_whitespace().map(|s| s.to_string()).collect()
}

pub fn parse(input: &str) -> Vec<Value> {
    let mut tokens = tokenize(input);
    let mut v = vec![];
    while !tokens.is_empty() {
        v.push(read_from_tokens(&mut tokens))
    }
    v
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Value {
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading");
    }

    let token = tokens.remove(0);
    if &token == "(" {
        let mut list = vec![];
        while &tokens[0] != ")" {
            list.push(read_from_tokens(tokens));
        }
        tokens.remove(0);
        return Value::List(Rc::new(list))
    } else if &token == ")" {
        panic!("Unexpected )");
    } else {
        atom(token)
    }
}

fn atom(s: String) -> Value {
    //TODO: handle everything other than ints :P
    match (s.parse(), s.parse(), s.parse(), s) {
        (Ok(i), _, _, _) => Value::Int(i),
        (_, Ok(f), _, _) => Value::Float(f),
        (_, _, Ok(b), _) => Value::Bool(b),
        (_, _, _, s) => {
            if s.starts_with("\"") {
                Value::String(Rc::new(s[1..s.len()-1].to_string()))
            } else if s.starts_with("'") {
                Value::new_list(
                    vec![
                        Value::new_ident("quote"),
                        Value::new_ident(s[1..].to_string())])
            } else {
                Value::new_ident(s)
            }
        }
    }
}
