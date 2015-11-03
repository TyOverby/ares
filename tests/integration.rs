extern crate ares;

use std::io::{Read, BufRead};
use std::fs::File;

use ares::{Context, user_fn};

#[test]
pub fn integration_tests() {
    run_test("values");
    run_test("fib");
    run_test("factorial");
    //run_test("iflet");
    run_test("optional");
}

fn get_lines(contents: String) -> (String, Vec<String>) {
    let mut program = vec![];
    let mut expected = vec![];
    let mut seen_sep = false;
    for line in contents.lines() {
        if line.len() > 4 && line.chars().all(|c| c == '=') {
            seen_sep = true;
        } else if seen_sep {
            expected.push(line.to_owned());
        } else {
            program.push(line.to_owned());
        }
    }
    (program.join("\n"), expected)
}

fn read_file(test_name: &str) -> String {
    let file = File::open(&format!("./tests/integration/{}.artest", test_name));
    let mut file = file.unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

fn run_test(test_name: &str) {
    let contents = read_file(test_name);
    let (program, expected) = get_lines(contents);

    let mut ctx: Context<Vec<String>> = Context::new();

    ctx.set_fn("print", user_fn("print", |args, ctx| {
        try!(ares::stdlib::util::expect_arity(args, |l| l == 1, "exactly 1"));
        let formatted = ctx.format_value(&args[0]);
        let state: &mut Vec<String> = ctx.state();
        state.push(formatted);
        Ok(0.into())
    }));

    let mut output = vec![];
    {
        let mut ctx = ctx.load(&mut output);
        ctx.eval_str(&program).unwrap();
    }

    assert_eq!(output, expected);
}
