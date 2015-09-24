extern crate ares;

use ::ares::*;

#[macro_export]
macro_rules! eval_ok {
    ($prog: expr, $v: expr) => {
        assert_eq!(util::e($prog).unwrap(), $v.into());
    }
}


pub fn e(program: &str) -> AresResult<Value> {
    let mut env = stdlib::basic_environment();
    let trees = parse(program).unwrap();
    let mut last = None;
    for tree in trees {
        last = Some(try!(eval(&tree, &mut env)))
    }
    Ok(last.expect("no program found"))
}
