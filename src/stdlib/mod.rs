use ::{user_fn, free_fn, ast_fn, Context, State};

// Keep these here for when you want to build huge changes
// pub fn load_all<T>(_: T) {}
// pub fn load_debug<T>(_: T) {}


pub mod arithmetic;
pub mod math;
pub mod core;
pub mod types;
pub mod list;
pub mod logical;
pub mod map;
pub mod debugger;

pub mod util {
    use ::{AresError, AresResult};
    pub fn expect_arity<F, S: Into<String>, T>(slice: &[T], expected: F, expect_str: S) -> AresResult<()>
    where S: Into<String>, F: FnOnce(usize) -> bool {
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

fn eval_into<S: State + ?Sized, P: AsRef<str>>(src: &P, ctx: &mut Context<S>) {
    use std::mem::uninitialized;
    let mut dummy: &mut S = unsafe { uninitialized() };
    let mut ctx = ctx.load(dummy);
    ctx.eval_str(src.as_ref()).unwrap();
}

pub fn load_all<S: State + ?Sized>(ctx: &mut Context<S>) {
    load_core(ctx);
    load_logical(ctx);
    load_list(ctx);
    load_math(ctx);
    load_arithmetic(ctx);
    load_map(ctx);
    load_types(ctx);
}

pub fn load_debug<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("debugger", ast_fn("debugger", self::debugger::debugger));
}

pub fn load_map<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("hash-map", ast_fn("hash-map", self::map::hash_map));
}

pub fn load_logical<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("and", ast_fn("and", self::logical::and));
    ctx.set_fn("or", ast_fn("or", self::logical::or));
    ctx.set_fn("xor", ast_fn("xor", self::logical::xor));
}

pub fn load_core<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("eval", user_fn("eval", self::core::eval));
    ctx.set_fn("apply", user_fn("apply", self::core::apply));
    ctx.set_fn("quote", ast_fn("quote", self::core::quote));
    ctx.set_fn("quasiquote", ast_fn("quasiquote", self::core::quasiquote));
    ctx.set_fn("macroexpand", user_fn("macroexpand", self::core::macroexpand));
    ctx.set_fn("unquote", ast_fn("unquote", self::core::unquote_error));
    ctx.set_fn("unquote-splicing", ast_fn("unquote-splicing", self::core::unquote_error));
    ctx.set_fn("if", ast_fn("if", self::core::cond));
    ctx.set_fn("let", ast_fn("let", self::core::lett));
    ctx.set_fn("set", ast_fn("set", self::core::set));
    ctx.set_fn("define", ast_fn("define", self::core::define));
    ctx.set_fn("define-macro", ast_fn("define-macro", self::core::define_macro));
    ctx.set_fn("lambda", ast_fn("lambda", self::core::lambda));
    ctx.set_fn("gensym", user_fn("gensym", self::core::gensym));
}

pub fn load_list<S: State + ?Sized>(ctx: &mut Context<S>) {
    {
        ctx.set_fn("build-list", ast_fn("build-list", self::list::build_list));
        ctx.set_fn("for-each", user_fn("for-each", self::list::foreach));
    }
    eval_into(&format!("(define list {})", self::list::LIST), ctx);
    eval_into(&format!("(define map {})", self::list::MAP), ctx);
    eval_into(&format!("(define fold-left {})", self::list::FOLD_LEFT), ctx);
    eval_into(&format!("(define filter {})", self::list::FILTER), ctx);
    eval_into(&format!("(define flatten {})", self::list::FLATTEN), ctx);
    eval_into(&format!("(define concat {})", self::list::CONCAT), ctx);
}

pub fn load_arithmetic<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("=", free_fn("=", self::core::equals));
    ctx.set_fn("+", free_fn("+", self::arithmetic::add_ints));
    ctx.set_fn("+.", free_fn("+.", self::arithmetic::add_floats));

    ctx.set_fn("-", free_fn("-", self::arithmetic::sub_ints));
    ctx.set_fn("-.", free_fn("-.", self::arithmetic::sub_floats));

    ctx.set_fn("*", free_fn("*", self::arithmetic::mul_ints));
    ctx.set_fn("*.", free_fn("*.", self::arithmetic::mul_floats));

    ctx.set_fn("/", free_fn("/", self::arithmetic::div_ints));
    ctx.set_fn("/.", free_fn("/.", self::arithmetic::div_floats));
}

pub fn load_math<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("nan?", free_fn("nan?", self::math::is_nan));
    ctx.set_fn("infinite?", free_fn("infinite?", self::math::is_infinite));
    ctx.set_fn("finite?", free_fn("finite?", self::math::is_finite));
    ctx.set_fn("normal?", free_fn("normal?", self::math::is_normal));

    ctx.set_fn("floor", free_fn("floor", self::math::floor));
    ctx.set_fn("ceil", free_fn("ceil", self::math::ceil));
    ctx.set_fn("round", free_fn("round", self::math::round));
    ctx.set_fn("trunc", free_fn("trunc", self::math::trunc));

    ctx.set_fn("fract", free_fn("fract", self::math::fract));
    ctx.set_fn("sign_positive?", free_fn("sign_positive?", self::math::is_sign_positive));
    ctx.set_fn("sign_negative?", free_fn("sign_negative?", self::math::is_sign_negative));
    ctx.set_fn("recip", free_fn("recip", self::math::recip));
    ctx.set_fn("sqrt", free_fn("sqrt", self::math::sqrt));
    ctx.set_fn("exp", free_fn("exp", self::math::exp));
    ctx.set_fn("exp2", free_fn("exp2", self::math::exp2));
    ctx.set_fn("ln", free_fn("ln", self::math::ln));
    ctx.set_fn("log2", free_fn("log2", self::math::log2));
    ctx.set_fn("log10", free_fn("log10", self::math::log10));
    ctx.set_fn("->degrees", free_fn("->degrees", self::math::to_degrees));
    ctx.set_fn("->radians", free_fn("->radians", self::math::to_radians));
    ctx.set_fn("cbrt", free_fn("cbrt", self::math::cbrt));
    ctx.set_fn("sin", free_fn("sin", self::math::sin));
    ctx.set_fn("cos", free_fn("cos", self::math::cos));
    ctx.set_fn("tan", free_fn("tan", self::math::tan));
    ctx.set_fn("asin", free_fn("asin", self::math::asin));
    ctx.set_fn("acos", free_fn("acos", self::math::acos));
    ctx.set_fn("atan", free_fn("atan", self::math::atan));
    ctx.set_fn("exp_m1", free_fn("exp_m1", self::math::exp_m1));
    ctx.set_fn("ln_1p", free_fn("ln_1p", self::math::ln_1p));
    ctx.set_fn("sinh", free_fn("sinh", self::math::sinh));
    ctx.set_fn("cosh", free_fn("cosh", self::math::cosh));
    ctx.set_fn("tanh", free_fn("tanh", self::math::tanh));
    ctx.set_fn("asinh", free_fn("asinh", self::math::asinh));
    ctx.set_fn("acosh", free_fn("acosh", self::math::acosh));
    ctx.set_fn("atanh", free_fn("atanh", self::math::atanh));

    ctx.set_fn("count_ones", free_fn("count_ones", self::math::count_ones));
    ctx.set_fn("count_zeros", free_fn("count_zeros", self::math::count_zeros));
    ctx.set_fn("leading_zeros", free_fn("leading_zeros", self::math::leading_zeros));
    ctx.set_fn("trailing_zeros", free_fn("trailing_zeros", self::math::trailing_zeros));
    ctx.set_fn("swap_bytes", free_fn("swap_bytes", self::math::swap_bytes));
    ctx.set_fn("->big-endian", free_fn("->big-endian", self::math::to_be));
    ctx.set_fn("->little-endian", free_fn("->little-endian", self::math::to_le));
    ctx.set_fn("abs", free_fn("abs", self::math::abs));
    ctx.set_fn("signum", free_fn("signum", self::math::signum));
    ctx.set_fn("positive?", free_fn("positive?", self::math::is_positive));
    ctx.set_fn("negative?", free_fn("negative?", self::math::is_negative));
}

pub fn load_types<S: State + ?Sized>(ctx: &mut Context<S>) {
    ctx.set_fn("->int", free_fn("->int", self::types::to_int));
    ctx.set_fn("->float", free_fn("->float", self::types::to_float));
    ctx.set_fn("->string", user_fn("->string", self::types::to_string));
    ctx.set_fn("->bool", free_fn("->bool", self::types::to_bool));

    ctx.set_fn("int?", free_fn("int?", self::types::is_int));
    ctx.set_fn("float?", free_fn("float?", self::types::is_float));
    ctx.set_fn("bool?", free_fn("bool?", self::types::is_bool));
    ctx.set_fn("string?", free_fn("string?", self::types::is_string));
    ctx.set_fn("list?", free_fn("list?", self::types::is_list));
    ctx.set_fn("lambda?", free_fn("lambda?", self::types::is_lambda));
    ctx.set_fn("foreign-fn?", free_fn("foreign-fn?", self::types::is_foreign_fn));
    ctx.set_fn("executable", free_fn("executable", self::types::is_executable));
}
