extern crate wisp;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut env = wisp::Environment::new();
    env.set_function("+", wisp::stdlib::add_ints);

    let env = Rc::new(RefCell::new(env));

    let program = wisp::parse("(+ 1 2 3)");

    println!("{:?}", wisp::eval(program, &env));
}
