use ::Environment;


pub mod arithmetic;
pub mod math;
pub mod core;
pub mod conv;

pub fn load_all(env: &mut Environment) {
    load_core(env);
    load_arithmetic(env);
    load_types(env);
}

pub fn load_core(env: &mut Environment) {
    env.set_uneval_function("quote", self::core::quote);
    env.set_uneval_function("if", self::core::cond);
    env.set_uneval_function("define", self::core::define);
    env.set_uneval_function("lambda", self::core::lambda);
}

pub fn load_arithmetic(env: &mut Environment) {
    env.set_function("=", self::core::equals);
    env.set_function("+", self::arithmetic::add_ints);
    env.set_function("+.", self::arithmetic::add_floats);

    env.set_function("-", self::arithmetic::sub_ints);
    env.set_function("-.", self::arithmetic::sub_floats);

    env.set_function("*", self::arithmetic::mul_ints);
    env.set_function("*.", self::arithmetic::mul_floats);

    env.set_function("/", self::arithmetic::div_ints);
    env.set_function("/.", self::arithmetic::div_floats);
}

pub fn load_types(env: &mut Environment) {
    env.set_function("->int", self::conv::to_int);
    env.set_function("->float", self::conv::to_float);
    env.set_function("->string", self::conv::to_string);
    env.set_function("->bool", self::conv::to_bool);

    env.set_function("int?", self::conv::is_int);
    env.set_function("float?", self::conv::is_float);
    env.set_function("bool?", self::conv::is_bool);
    env.set_function("string?", self::conv::is_string);
    env.set_function("list?", self::conv::is_list);
    env.set_function("lambda?", self::conv::is_lambda);
    env.set_function("foreign-fn?", self::conv::is_foreign_fn);
    env.set_function("executable", self::conv::is_executable);
}
