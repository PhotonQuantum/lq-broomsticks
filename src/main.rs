#![feature(box_syntax)]
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;
mod indice;

use crate::parser::parse;
use indice::uid::{to_uid, UIDGenerator};

fn main() {
    test_uid("λf.(λx.f (x x)) (λx.f (x x))");
    test_uid("λx.xx");
}

fn test_uid(expr: &str) {
    let expr = parse(expr).unwrap();
    println!("{:?}", expr);

    let mut gen = UIDGenerator::default();
    let uid_expr = to_uid(&expr, &mut gen);
    println!("{:?}", uid_expr);
    println!("{}", uid_expr.to_bare());
}