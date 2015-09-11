extern crate rebar;

use ::rebar::*;

use std::rc::Rc;
use std::cell::RefCell;

fn basic_environment() -> Rc<RefCell<Environment>> {
    let mut env = Environment::new();
    env.set_function("+", stdlib::add_ints);
    env.set_function("+.", stdlib::add_floats);

    env.set_function("-", stdlib::sub_ints);
    env.set_function("-.", stdlib::sub_floats);

    env.set_function("*", stdlib::mul_ints);
    env.set_function("*.", stdlib::mul_floats);

    env.set_function("/", stdlib::div_ints);
    env.set_function("/.", stdlib::div_floats);
    Rc::new(RefCell::new(env))
}

pub fn e(program: &str) -> Value {
    eval(&parse(program), &basic_environment())
}
