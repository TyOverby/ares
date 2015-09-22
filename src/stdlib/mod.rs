use ::Environment;

pub mod arithmetic;
pub mod math;
pub mod core;
pub mod types;
pub mod list;

pub fn load_all(env: &mut Environment) {
    load_core(env);
    load_list(env);
    load_math(env);
    load_arithmetic(env);
    load_types(env);
}

pub fn load_core(env: &mut Environment) {
    env.set_uneval_function("quote", self::core::quote);
    env.set_uneval_function("if", self::core::cond);
    env.set_uneval_function("define", self::core::define);
    env.set_uneval_function("lambda", self::core::lambda);
}

pub fn load_list(env: &mut Environment) {
    env.set_uneval_function("build-list", self::list::build_list);
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

pub fn load_math(env: &mut Environment) {
    env.set_function("nan?", self::math::is_nan);
    env.set_function("infinite?", self::math::is_infinite);
    env.set_function("finite?", self::math::is_finite);
    env.set_function("normal?", self::math::is_normal);

    env.set_function("floor", self::math::floor);
    env.set_function("ceil", self::math::ceil);
    env.set_function("round", self::math::round);
    env.set_function("trunc", self::math::trunc);

    env.set_function("fract", self::math::fract);
    env.set_function("sign_positive?", self::math::is_sign_positive);
    env.set_function("sign_negative?", self::math::is_sign_negative);
    env.set_function("recip", self::math::recip);
    env.set_function("sqrt", self::math::sqrt);
    env.set_function("exp", self::math::exp);
    env.set_function("exp2", self::math::exp2);
    env.set_function("ln", self::math::ln);
    env.set_function("log2", self::math::log2);
    env.set_function("log10", self::math::log10);
    env.set_function("->degrees", self::math::to_degrees);
    env.set_function("->radians", self::math::to_radians);
    env.set_function("cbrt", self::math::cbrt);
    env.set_function("sin", self::math::sin);
    env.set_function("cos", self::math::cos);
    env.set_function("tan", self::math::tan);
    env.set_function("asin", self::math::asin);
    env.set_function("acos", self::math::acos);
    env.set_function("atan", self::math::atan);
    env.set_function("exp_m1", self::math::exp_m1);
    env.set_function("ln_1p", self::math::ln_1p);
    env.set_function("sinh", self::math::sinh);
    env.set_function("cosh", self::math::cosh);
    env.set_function("tanh", self::math::tanh);
    env.set_function("asinh", self::math::asinh);
    env.set_function("acosh", self::math::acosh);
    env.set_function("atanh", self::math::atanh);

    env.set_function("count_ones", self::math::count_ones);
    env.set_function("count_zeros", self::math::count_zeros);
    env.set_function("leading_zeros", self::math::leading_zeros);
    env.set_function("trailing_zeros", self::math::trailing_zeros);
    env.set_function("swap_bytes", self::math::swap_bytes);
    env.set_function("->big-endian", self::math::to_be);
    env.set_function("->little-endian", self::math::to_le);
    env.set_function("abs", self::math::abs);
    env.set_function("signum", self::math::signum);
    env.set_function("positive?", self::math::is_positive);
    env.set_function("negative?", self::math::is_negative);
}

pub fn load_types(env: &mut Environment) {
    env.set_function("->int", self::types::to_int);
    env.set_function("->float", self::types::to_float);
    env.set_function("->string", self::types::to_string);
    env.set_function("->bool", self::types::to_bool);

    env.set_function("int?", self::types::is_int);
    env.set_function("float?", self::types::is_float);
    env.set_function("bool?", self::types::is_bool);
    env.set_function("string?", self::types::is_string);
    env.set_function("list?", self::types::is_list);
    env.set_function("lambda?", self::types::is_lambda);
    env.set_function("foreign-fn?", self::types::is_foreign_fn);
    env.set_function("executable", self::types::is_executable);
}
