extern crate rebar;

use ::rebar::*;

use std::rc::Rc;
use std::cell::RefCell;

fn basic_environment() -> Rc<RefCell<Environment>> {
    let mut env = Environment::new();
    env.set_function("+", stdlib::arithmetic::add_ints);
    env.set_function("+.", stdlib::arithmetic::add_floats);

    env.set_function("-", stdlib::arithmetic::sub_ints);
    env.set_function("-.", stdlib::arithmetic::sub_floats);

    env.set_function("*", stdlib::arithmetic::mul_ints);
    env.set_function("*.", stdlib::arithmetic::mul_floats);

    env.set_function("/", stdlib::arithmetic::div_ints);
    env.set_function("/.", stdlib::arithmetic::div_floats);
    Rc::new(RefCell::new(env))
}

pub fn e(program: &str) -> Value {
    eval(&parse(program), &basic_environment())
}
