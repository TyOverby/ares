// Based on Norvig's lisp interpreter
use std::rc::Rc;

use super::Value;

fn tokenize(s: &str) -> Vec<String> {
    s.replace("(", " ( ").replace(")", " ) ").split_whitespace().map(|s| s.to_string()).collect()
}

pub fn parse(input: &str) -> Value {
    let mut tokens = tokenize(input);
    read_from_tokens(&mut tokens)
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Value {
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading");
    }

    let mut token = tokens.remove(0);
    if &token == "(" {
        let mut list = vec![];
        while &tokens[0] != ")" {
            list.push(Rc::new(read_from_tokens(tokens)));
        }
        tokens.remove(0);
        return Value::List(list)
    } else if &token == ")" {
        panic!("Unexpected )");
    } else {
        atom(token)
    }
}

fn atom(s: String) -> Value {
    //TODO: handle everything other than ints :P
    match s.parse() {
        Ok(i) => Value::Int(i),
        Err(_) => Value::Ident(s)
    }
}
