#![feature(box_syntax)]
#![feature(iterator_fold_self)]
#[macro_use]
extern crate pest_derive;

use ast::ToBareTerm;
use indice::uid::{to_uid, UIDGenerator};

use crate::ast::Reducible;
use crate::parser::parse;

mod ast;
mod indice;
mod parser;

fn main() {
    test_reduce("(λa.λb.λc.a (λd.λe.e (d b)) (λd.c) (λd.d)) (λa.λb.a b)");
    test_reduce("(λf.(λx.f (x x)) (λx.f (x x))) \\f.x");
    test_reduce("(\\y.\\x.yxz)x");
}

fn test_reduce(expr: &str) {
    let expr = parse(expr).unwrap();
    let expr = to_uid(&expr, &mut UIDGenerator::default());
    println!("{} -->nor {}", expr.to_bare(), expr.nor_reduce().to_bare());
}
