extern crate ares;

use ares::{Context, user_fn};

fn main() {
    // This context is a Context<bool>
    let mut context = Context::new();
    // So we can write functions that expect a boolean state
    context.set_fn("trigger", user_fn("trigger", |_, ctx| {
        *ctx.state() = true;
        Ok(true.into())
    }));

    for _ in 0 .. 10 {
        // A new state for each loop!
        let mut triggered = false;
        {
            // The context gets loaded once for every iteration
            let mut loaded = context.load(&mut triggered);
            // Call the function that we loaded in the bare Context
            loaded.eval_str("(trigger)").unwrap();
        }
        // Our local state gets modified
        assert!(triggered);
    }
}
