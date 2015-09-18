extern crate ares;
extern crate term_painter;

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, BufRead};

use term_painter::Color::*;
use term_painter::ToStyle;

fn main() {
    let mut env = ares::Environment::new();
    ares::stdlib::load_all(&mut env);
    let mut env = Rc::new(RefCell::new(env));

    let stdin = io::stdin();
    for line in stdin.lock().lines().take_while(|a| a.is_ok()).filter_map(|a| a.ok()) {
        for tree in ares::parse(&line) {
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
