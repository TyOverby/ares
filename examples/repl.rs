extern crate ares;
extern crate term_painter;

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, BufRead};

use term_painter::Color::*;
use term_painter::ToStyle;
use term_painter::Attr::*;

fn main() {
    let mut env = ares::Environment::new();
    ares::stdlib::load_all(&mut env);
    let mut env = Rc::new(RefCell::new(env));
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let trees = match ares::parse(&buf) {
            Ok(trees) => trees,
            Err(parse_error) => {
                println!("Parse error: {}", Red.paint(parse_error));
                continue;
            }
        };
        let mut last = None;
        for tree in trees {
            let evald = ares::eval(&tree, &mut env);
            let erred = evald.is_err();
            last = Some(evald);
            if erred { break; }
        }

        match last {
            Some(Ok(v)) => println!("{:?}", Green.paint(v)),
            Some(Err(e)) => println!("err: {:?}", Red.paint(e)),
            None => {}
        }
    }
}
