use ::{free_fn, ast_fn, Context, LoadedContext};

pub mod arithmetic;
pub mod math;
pub mod core;
pub mod types;
pub mod list;
pub mod logical;

pub mod util {
    use ::{AresError, AresResult};
    pub fn expect_arity<F, E, T, S>(slice: &[T], expected: F, expect_str: E) -> AresResult<(), S>
    where E: Into<String>, F: FnOnce(usize) -> bool {
        let len = slice.len();
        if expected(len) {
            Ok(())
        } else {
            Err(AresError::UnexpectedArity{
                found: len as u16,
                expected: expect_str.into()
            })
        }
    }
}

unsafe fn load_fake<'a, T: 'static>(ctx: &'a mut Context<T>) -> LoadedContext<'a, T> {
    use std::mem::transmute;
    use ::{eval, parse};

    let dummy = ();
    let dumb_ref: &'a mut T = unsafe { transmute(&dummy) };

    ctx.load(dumb_ref)
}

fn eval_into<'a, T: 'static, S: AsRef<str>>(src: &S, ctx: &'a mut Context<T>) {
    use std::mem::transmute;
    use ::{eval, parse};

    let mut ctx = unsafe { load_fake::<T>(ctx) };

    let parsed = parse(src.as_ref()).unwrap();
    for statement in &parsed {
        ctx.eval(statement).unwrap();
    }
}

pub fn load_all<T: 'static>(ctx: &mut Context<T>) {
    load_logical(ctx);
    load_core(ctx);
    load_list(ctx);
    load_math(ctx);
    load_arithmetic(ctx);
    load_types(ctx);
}

pub fn load_logical<T: 'static>(ctx: &mut Context<T>) {
    ctx.set("and", ast_fn("and", self::logical::and));
    ctx.set("or", ast_fn("or", self::logical::or));
    ctx.set("xor", ast_fn("xor", self::logical::xor));
}

pub fn load_core<T: 'static>(ctx: &mut Context<T>) {
    ctx.set("quote", free_fn("quote", self::core::quote));
    ctx.set("if", ast_fn("if", self::core::cond));
    ctx.set("set", ast_fn("set", self::core::set));
    ctx.set("define", ast_fn("define", self::core::define));
    ctx.set("lambda", ast_fn("lambda", self::core::lambda));
}

pub fn load_list<T: 'static>(ctx: &mut Context<T>) {
    {
        ctx.set("build-list", ast_fn("build-list", self::list::build_list));
        ctx.set("for-each", ast_fn("for-each", self::list::foreach));
    }
    eval_into(&format!("(define list {})", self::list::LIST), ctx);
    eval_into(&format!("(define map {})", self::list::MAP), ctx);
    eval_into(&format!("(define fold-left {})", self::list::FOLD_LEFT), ctx);
    eval_into(&format!("(define filter {})", self::list::FILTER), ctx);
    eval_into(&format!("(define concat {})", self::list::CONCAT), ctx);

}

pub fn load_arithmetic<T: 'static>(ctx: &mut Context<T>) {
    ctx.set("=", free_fn("=", self::core::equals));
    ctx.set("+", free_fn("+", self::arithmetic::add_ints));
    ctx.set("+.", free_fn("+.", self::arithmetic::add_floats));

    ctx.set("-", free_fn("-", self::arithmetic::sub_ints));
    ctx.set("-.", free_fn("-.", self::arithmetic::sub_floats));

    ctx.set("*", free_fn("*", self::arithmetic::mul_ints));
    ctx.set("*.", free_fn("*.", self::arithmetic::mul_floats));

    ctx.set("/", free_fn("/", self::arithmetic::div_ints));
    ctx.set("/.", free_fn("/.", self::arithmetic::div_floats));
}

pub fn load_math<T: 'static>(ctx: &mut Context<T>) {
    ctx.set("nan?", free_fn("nan?", self::math::is_nan));
    ctx.set("infinite?", free_fn("infinite?", self::math::is_infinite));
    ctx.set("finite?", free_fn("finite?", self::math::is_finite));
    ctx.set("normal?", free_fn("normal?", self::math::is_normal));

    ctx.set("floor", free_fn("floor", self::math::floor));
    ctx.set("ceil", free_fn("ceil", self::math::ceil));
    ctx.set("round", free_fn("round", self::math::round));
    ctx.set("trunc", free_fn("trunc", self::math::trunc));

    ctx.set("fract", free_fn("fract", self::math::fract));
    ctx.set("sign_positive?", free_fn("sign_positive?", self::math::is_sign_positive));
    ctx.set("sign_negative?", free_fn("sign_negative?", self::math::is_sign_negative));
    ctx.set("recip", free_fn("recip", self::math::recip));
    ctx.set("sqrt", free_fn("sqrt", self::math::sqrt));
    ctx.set("exp", free_fn("exp", self::math::exp));
    ctx.set("exp2", free_fn("exp2", self::math::exp2));
    ctx.set("ln", free_fn("ln", self::math::ln));
    ctx.set("log2", free_fn("log2", self::math::log2));
    ctx.set("log10", free_fn("log10", self::math::log10));
    ctx.set("->degrees", free_fn("->degrees", self::math::to_degrees));
    ctx.set("->radians", free_fn("->radians", self::math::to_radians));
    ctx.set("cbrt", free_fn("cbrt", self::math::cbrt));
    ctx.set("sin", free_fn("sin", self::math::sin));
    ctx.set("cos", free_fn("cos", self::math::cos));
    ctx.set("tan", free_fn("tan", self::math::tan));
    ctx.set("asin", free_fn("asin", self::math::asin));
    ctx.set("acos", free_fn("acos", self::math::acos));
    ctx.set("atan", free_fn("atan", self::math::atan));
    ctx.set("exp_m1", free_fn("exp_m1", self::math::exp_m1));
    ctx.set("ln_1p", free_fn("ln_1p", self::math::ln_1p));
    ctx.set("sinh", free_fn("sinh", self::math::sinh));
    ctx.set("cosh", free_fn("cosh", self::math::cosh));
    ctx.set("tanh", free_fn("tanh", self::math::tanh));
    ctx.set("asinh", free_fn("asinh", self::math::asinh));
    ctx.set("acosh", free_fn("acosh", self::math::acosh));
    ctx.set("atanh", free_fn("atanh", self::math::atanh));

    ctx.set("count_ones", free_fn("count_ones", self::math::count_ones));
    ctx.set("count_zeros", free_fn("count_zeros", self::math::count_zeros));
    ctx.set("leading_zeros", free_fn("leading_zeros", self::math::leading_zeros));
    ctx.set("trailing_zeros", free_fn("trailing_zeros", self::math::trailing_zeros));
    ctx.set("swap_bytes", free_fn("swap_bytes", self::math::swap_bytes));
    ctx.set("->big-endian", free_fn("->big-endian", self::math::to_be));
    ctx.set("->little-endian", free_fn("->little-endian", self::math::to_le));
    ctx.set("abs", free_fn("abs", self::math::abs));
    ctx.set("signum", free_fn("signum", self::math::signum));
    ctx.set("positive?", free_fn("positive?", self::math::is_positive));
    ctx.set("negative?", free_fn("negative?", self::math::is_negative));
}

pub fn load_types<T: 'static>(ctx: &mut Context<T>) {
    ctx.set("->int", free_fn("->int", self::types::to_int));
    ctx.set("->float", free_fn("->float", self::types::to_float));
    ctx.set("->string", free_fn("->string", self::types::to_string));
    ctx.set("->bool", free_fn("->bool", self::types::to_bool));

    ctx.set("int?", free_fn("int?", self::types::is_int));
    ctx.set("float?", free_fn("float?", self::types::is_float));
    ctx.set("bool?", free_fn("bool?", self::types::is_bool));
    ctx.set("string?", free_fn("string?", self::types::is_string));
    ctx.set("list?", free_fn("list?", self::types::is_list));
    ctx.set("lambda?", free_fn("lambda?", self::types::is_lambda));
    ctx.set("foreign-fn?", free_fn("foreign-fn?", self::types::is_foreign_fn));
    ctx.set("executable", free_fn("executable", self::types::is_executable));
}
