extern crate ares;
extern crate term_painter;

use term_painter::Color::*;
use term_painter::ToStyle;

fn main() {
    let mut ctx = ares::Context::new().with_debug();
    let mut dummy = ();
    let mut ctx = ctx.load(&mut dummy);

    while let Some(line) = ares::util::prompt("repl> ") {
        match ctx.eval_str(&line) {
            Ok(v)  => println!("{}", Green.paint(ctx.format_value(&v))),
            Err(e) => println!("{:?}", Red.paint(e))
        }
    }
}
