extern crate rebar;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut env = rebar::Environment::new();
    env.set_function("+", rebar::stdlib::add_ints);

    let env = Rc::new(RefCell::new(env));

    let program = rebar::parse("(+ 1 2 3)");

    println!("{:?}", rebar::eval(&program, &env));
}
