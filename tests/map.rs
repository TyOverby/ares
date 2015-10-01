extern crate ares;
use std::collections::HashMap;

#[macro_use]
mod util;

macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => ({
        let mut m = HashMap::new();
        $(m.insert($k, $v));*;
        m
    })
}

#[test]
fn test_hashmap() {
    eval_ok!("(hash-map 1 2 3 4)", hashmap!(1 => 2, 3 => 4));
    eval_ok!("(define x 2) {1 x 3 4}", hashmap!(1 => 2, 3 => 4));
    eval_ok!("{1 2 3 4}", hashmap!(1 => 2, 3 => 4));
}
