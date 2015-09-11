use ::Value;

macro_rules! gen_num_method {
    ($name: ident, $v: path) => {
        gen_num_method!($name, $v, $v);
    };
    ($name: ident, $inv: path, $outv: path) => {
        gen_num_method!($name, $inv, $outv, |a| a);
    };
    ($name: ident, $inv: path, $outv: path, $conv: expr) => {
        pub fn $name(it: &mut Iterator<Item=Value>) -> Value {
            let value = match it.next() {
                Some($inv(v)) => $outv($conv(v.$name())),
                Some(_) => panic!("unexpected math arg"),
                None => panic!("bad args list")
            };
            if it.next().is_some() {
                panic!("too many arguments");
            }
            value
        }
    }
}

gen_num_method!(is_nan, Value::Float, Value::Bool);
gen_num_method!(is_infinite, Value::Float, Value::Bool);
gen_num_method!(is_finite, Value::Float, Value::Bool);
gen_num_method!(is_normal, Value::Float, Value::Bool);

gen_num_method!(floor, Value::Float, Value::Int, |a| a as i64);
gen_num_method!(ceil, Value::Float, Value::Int, |a| a as i64);
gen_num_method!(round, Value::Float, Value::Int, |a| a as i64);
gen_num_method!(trunc, Value::Float, Value::Int, |a| a as i64);

gen_num_method!(fract, Value::Float);
gen_num_method!(is_sign_positive, Value::Float, Value::Bool);
gen_num_method!(is_sign_negative, Value::Float, Value::Bool);
gen_num_method!(recip, Value::Float);
gen_num_method!(sqrt, Value::Float);
gen_num_method!(exp, Value::Float);
gen_num_method!(exp2, Value::Float);
gen_num_method!(ln, Value::Float);
gen_num_method!(log2, Value::Float);
gen_num_method!(log10, Value::Float);
gen_num_method!(to_degrees, Value::Float);
gen_num_method!(to_radians, Value::Float);
gen_num_method!(cbrt, Value::Float);
gen_num_method!(sin, Value::Float);
gen_num_method!(cos, Value::Float);
gen_num_method!(tan, Value::Float);
gen_num_method!(asin, Value::Float);
gen_num_method!(acos, Value::Float);
gen_num_method!(atan, Value::Float);
gen_num_method!(exp_m1, Value::Float);
gen_num_method!(ln_1p, Value::Float);
gen_num_method!(sinh, Value::Float);
gen_num_method!(cosh, Value::Float);
gen_num_method!(tanh, Value::Float);
gen_num_method!(asinh, Value::Float);
gen_num_method!(acosh, Value::Float);
gen_num_method!(atanh, Value::Float);



// TODO:
// powf
// powi
// log
// max
// min
// hypot
// sin_cos
// atan2

