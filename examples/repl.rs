extern crate ares;
extern crate term_painter;

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, BufRead};

use term_painter::Color::*;
use term_painter::ToStyle;

fn main() {
    let env = ares::Environment::new();
    let mut env = Rc::new(RefCell::new(env));
    ares::stdlib::load_all(&env);

    let stdin = io::stdin();
    for line in stdin.lock().lines().take_while(|a| a.is_ok()).filter_map(|a| a.ok()) {
        let trees = match ares::parse(&line) {
            Ok(trees) => trees,
            Err(parse_error) => {
                println!("Parse error: {}", Red.paint(parse_error));
                continue
            }
        };

        for tree in trees {
            match ares::eval(&tree, &mut env) {
                Ok(v) => {
                    println!("{:?}", Green.paint(v))
                }
                Err(e) => {
                    println!("err: {:?}", Red.paint(e));
                    break;
                }
            }
        }
    }
}
