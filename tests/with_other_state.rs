extern crate ares;
use ares::*;

#[test]
fn basic_swap() {
    let mut context = Context::new();
    context.set_fn("run", user_fn("run", |_args, ctx| {
        *ctx.state() = *ctx.state() + 1;
        Ok(Value::Int(*ctx.state()))
   }));

    let mut outer = 0i64;
    let mut inner = 0i64;
    {
        let mut loaded = context.load(&mut outer);
        assert_eq!(loaded.eval_str("(run)").unwrap(), 1.into());

        loaded.with_other_state(&mut inner, |new_loaded| {
            assert_eq!(new_loaded.eval_str("(run)").unwrap(), 1.into());
        });
    }
    assert_eq!(outer, 1);
    assert_eq!(inner, 1);
}
