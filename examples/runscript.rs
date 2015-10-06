extern crate ares;

use ares::{Value, Context, free_fn};
use std::io::{Read, stdin};
use std::fmt::Write;

fn main() {
    let mut ctx = Context::new().with_debug();

    ctx.set_fn("print", free_fn("print", |args| {
        let mut buf = String::new();
        write!(buf, "{:?}", args);
        println!("{}", buf);
        Ok(Value::string(buf))
    }));

    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer);

    ctx.eval_str(&buffer);
}
