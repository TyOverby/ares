use ::{Env, free_fn, ast_fn};

pub mod arithmetic;
pub mod math;
pub mod core;
pub mod types;
pub mod list;
pub mod logical;

pub mod util {
    use ::AresError;

    pub fn unwrap_or_arity_err<T, S>(value: Option<T>, seen_already: u16, expected: S) -> Result<T, AresError>
    where S: Into<String> {
        match value {
            Some(v) => Ok(v),
            None => Err(AresError::UnexpectedArity {
                found: seen_already,
                expected: expected.into()
            })
        }
    }

    pub fn no_more_or_arity_err<S, T, I: ?Sized>(iter: &mut I, seen_already: u16, expected: S) -> Result<(), AresError>
    where I: Iterator<Item=T>, S: Into<String>
    {
        let count = iter.count();
        if count > 0 {
            Err(AresError::UnexpectedArity {
                found: seen_already + count as u16,
                expected: expected.into()
            })
        } else {
            Ok(())
        }
    }

}

fn eval_into<S: AsRef<str>>(src: &S, env: &Env) {
    use ::{eval, parse};
    let parsed = parse(src.as_ref()).unwrap();
    for statement in &parsed {
        eval(statement, env).unwrap();
    }
}

pub fn load_all(env: &Env) {
    load_logical(env);
    load_core(env);
    load_list(env);
    load_math(env);
    load_arithmetic(env);
    load_types(env);
}

pub fn load_logical(env: &Env) {
    let mut env = env.borrow_mut();
    env.insert_here("and", ast_fn("and", self::logical::and));
    env.insert_here("or", ast_fn("or", self::logical::or));
    env.insert_here("xor", ast_fn("xor", self::logical::xor));
}

pub fn load_core(env: &Env) {
    let mut env = env.borrow_mut();
    env.insert_here("quote", ast_fn("quote", self::core::quote));
    env.insert_here("if", ast_fn("if", self::core::cond));
    env.insert_here("set", ast_fn("set", self::core::set));
    env.insert_here("define", ast_fn("define", self::core::define));
    env.insert_here("lambda", ast_fn("lambda", self::core::lambda));
}

pub fn load_list(env: &Env) {
    {
        let mut env = env.borrow_mut();
        env.insert_here("build-list", ast_fn("build-list", self::list::build_list));
        env.insert_here("for-each", ast_fn("for-each", self::list::foreach));
    }
    eval_into(&format!("(define list {})", self::list::LIST), env);
    eval_into(&format!("(define map {})", self::list::MAP), env);
    eval_into(&format!("(define fold-left {})", self::list::FOLD_LEFT), env);
    eval_into(&format!("(define filter {})", self::list::FILTER), env);

}

pub fn load_arithmetic(env: &Env) {
    let mut env = env.borrow_mut();
    env.insert_here("=", free_fn("=", self::core::equals));
    env.insert_here("+", free_fn("+", self::arithmetic::add_ints));
    env.insert_here("+.", free_fn("+.", self::arithmetic::add_floats));

    env.insert_here("-", free_fn("-", self::arithmetic::sub_ints));
    env.insert_here("-.", free_fn("-.", self::arithmetic::sub_floats));

    env.insert_here("*", free_fn("*", self::arithmetic::mul_ints));
    env.insert_here("*.", free_fn("*.", self::arithmetic::mul_floats));

    env.insert_here("/", free_fn("/", self::arithmetic::div_ints));
    env.insert_here("/.", free_fn("/.", self::arithmetic::div_floats));
}

pub fn load_math(env: &Env) {
    let mut env = env.borrow_mut();
    env.insert_here("nan?", free_fn("nan?", self::math::is_nan));
    env.insert_here("infinite?", free_fn("infinite?", self::math::is_infinite));
    env.insert_here("finite?", free_fn("finite?", self::math::is_finite));
    env.insert_here("normal?", free_fn("normal?", self::math::is_normal));

    env.insert_here("floor", free_fn("floor", self::math::floor));
    env.insert_here("ceil", free_fn("ceil", self::math::ceil));
    env.insert_here("round", free_fn("round", self::math::round));
    env.insert_here("trunc", free_fn("trunc", self::math::trunc));

    env.insert_here("fract", free_fn("fract", self::math::fract));
    env.insert_here("sign_positive?", free_fn("sign_positive?", self::math::is_sign_positive));
    env.insert_here("sign_negative?", free_fn("sign_negative?", self::math::is_sign_negative));
    env.insert_here("recip", free_fn("recip", self::math::recip));
    env.insert_here("sqrt", free_fn("sqrt", self::math::sqrt));
    env.insert_here("exp", free_fn("exp", self::math::exp));
    env.insert_here("exp2", free_fn("exp2", self::math::exp2));
    env.insert_here("ln", free_fn("ln", self::math::ln));
    env.insert_here("log2", free_fn("log2", self::math::log2));
    env.insert_here("log10", free_fn("log10", self::math::log10));
    env.insert_here("->degrees", free_fn("->degrees", self::math::to_degrees));
    env.insert_here("->radians", free_fn("->radians", self::math::to_radians));
    env.insert_here("cbrt", free_fn("cbrt", self::math::cbrt));
    env.insert_here("sin", free_fn("sin", self::math::sin));
    env.insert_here("cos", free_fn("cos", self::math::cos));
    env.insert_here("tan", free_fn("tan", self::math::tan));
    env.insert_here("asin", free_fn("asin", self::math::asin));
    env.insert_here("acos", free_fn("acos", self::math::acos));
    env.insert_here("atan", free_fn("atan", self::math::atan));
    env.insert_here("exp_m1", free_fn("exp_m1", self::math::exp_m1));
    env.insert_here("ln_1p", free_fn("ln_1p", self::math::ln_1p));
    env.insert_here("sinh", free_fn("sinh", self::math::sinh));
    env.insert_here("cosh", free_fn("cosh", self::math::cosh));
    env.insert_here("tanh", free_fn("tanh", self::math::tanh));
    env.insert_here("asinh", free_fn("asinh", self::math::asinh));
    env.insert_here("acosh", free_fn("acosh", self::math::acosh));
    env.insert_here("atanh", free_fn("atanh", self::math::atanh));

    env.insert_here("count_ones", free_fn("count_ones", self::math::count_ones));
    env.insert_here("count_zeros", free_fn("count_zeros", self::math::count_zeros));
    env.insert_here("leading_zeros", free_fn("leading_zeros", self::math::leading_zeros));
    env.insert_here("trailing_zeros", free_fn("trailing_zeros", self::math::trailing_zeros));
    env.insert_here("swap_bytes", free_fn("swap_bytes", self::math::swap_bytes));
    env.insert_here("->big-endian", free_fn("->big-endian", self::math::to_be));
    env.insert_here("->little-endian", free_fn("->little-endian", self::math::to_le));
    env.insert_here("abs", free_fn("abs", self::math::abs));
    env.insert_here("signum", free_fn("signum", self::math::signum));
    env.insert_here("positive?", free_fn("positive?", self::math::is_positive));
    env.insert_here("negative?", free_fn("negative?", self::math::is_negative));
}

pub fn load_types(env: &Env) {
    let mut env = env.borrow_mut();
    env.insert_here("->int", free_fn("->int", self::types::to_int));
    env.insert_here("->float", free_fn("->float", self::types::to_float));
    env.insert_here("->string", free_fn("->string", self::types::to_string));
    env.insert_here("->bool", free_fn("->bool", self::types::to_bool));

    env.insert_here("int?", free_fn("int?", self::types::is_int));
    env.insert_here("float?", free_fn("float?", self::types::is_float));
    env.insert_here("bool?", free_fn("bool?", self::types::is_bool));
    env.insert_here("string?", free_fn("string?", self::types::is_string));
    env.insert_here("list?", free_fn("list?", self::types::is_list));
    env.insert_here("lambda?", free_fn("lambda?", self::types::is_lambda));
    env.insert_here("foreign-fn?", free_fn("foreign-fn?", self::types::is_foreign_fn));
    env.insert_here("executable", free_fn("executable", self::types::is_executable));
}
