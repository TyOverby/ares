extern crate ares;

#[macro_use]
mod util;

#[test]
fn convert_to_int() {
    eval_ok!("(->int 5)", 5);
    eval_ok!("(->int 1.2)", 1);
    eval_ok!("(->int \"10\")", 10);
    eval_ok!("(->int true)", 1);
    eval_ok!("(->int false)", 0);
}

#[test]
fn convert_to_float() {
    eval_ok!("(->float 5)", 5.0);
    eval_ok!("(->float 1.2)", 1.2);
    eval_ok!("(->float \"10\")", 10.0);
}

#[test]
fn convert_to_bool() {
    eval_ok!("(->bool true)", true);
    eval_ok!("(->bool false)", false);
    eval_ok!("(->bool 1)", true);
    eval_ok!("(->bool 0)", false);
}

#[test]
fn convert_to_string() {
    eval_ok!("(->string true)", "true");
    eval_ok!("(->string false)", "false");
    eval_ok!("(->string 1)", "1");
    eval_ok!("(->string 0)", "0");
    eval_ok!("(->string 1.5)", "1.5");
    eval_ok!("(->string \"hello\")", "hello");
}

#[test]
fn is_int() {
    eval_ok!("(int? 1)", true);

    eval_ok!("(int? 1.0)", false);
    eval_ok!("(int? true)", false);
    eval_ok!("(int? false)", false);
    eval_ok!("(int? \"hi\")", false);
}

#[test]
fn is_float() {
    eval_ok!("(float? 1.0)", true);

    eval_ok!("(float? 1)", false);
    eval_ok!("(float? true)", false);
    eval_ok!("(float? false)", false);
    eval_ok!("(float? \"hi\")", false);
}

#[test]
fn is_bool() {
    eval_ok!("(bool? true)", true);
    eval_ok!("(bool? false)", true);

    eval_ok!("(bool? 1)", false);
    eval_ok!("(bool? 1.0)", false);
    eval_ok!("(bool? \"hi\")", false);
}

// TODO: is_string is_list is_ident is_lambda is_foreign_fn
