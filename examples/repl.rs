extern crate ares;
extern crate term_painter;

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, BufRead};

use term_painter::Color::*;
use term_painter::ToStyle;

fn main() {
    let mut ctx = ares::Context::<()>::new();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    let stdin = io::stdin();
    for line in stdin.lock().lines().take_while(|a| a.is_ok()).filter_map(|a| a.ok()) {
        match ctx.eval_str(&line) {
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
