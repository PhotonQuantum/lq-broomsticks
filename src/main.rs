#![feature(box_syntax)]
#![feature(iterator_fold_self, type_alias_impl_trait)]
#[macro_use]
extern crate pest_derive;

use index::uid::*;

use crate::ast::ReduceStrategy::*;
use crate::ast::{BareID, ReduceStrategy, Reducible, Term};
use crate::parser::parse;

mod ast;
mod index;
mod parser;

fn main() {
    test_reduce(
        "(λa.λb.λc.a (λd.λe.e (d b)) (λd.c) (λd.d)) (λa.λb.a b)",
        APP,
    );
    test_reduce(
        "(λa.λb.λc.a (λd.λe.e (d b)) (λd.c) (λd.d)) (λa.λb.a b)",
        CBN,
    );
    test_reduce("(λf.(λx.f (x x)) (λx.f (x x))) \\f.x", APP);
    test_reduce("(λf.(λx.f (x x)) (λx.f (x x))) \\f.x", CBN);
    test_reduce("(\\y.\\x.zxy)x", APP);
    test_reduce("(\\y.\\x.zxy)x", CBN);
}

fn test_reduce(expr: &str, strategy: ReduceStrategy) {
    let expr = parse(expr).unwrap();
    let mut expr = from::to_uid(&expr, &mut UIDGenerator::default());
    println!("{}", Term::<BareID>::from(expr.clone()));
    loop {
        let new_expr = expr.beta_reduce(strategy, Some(1));
        if expr == new_expr {
            break;
        }
        println!("--> {}", Term::<BareID>::from(new_expr.clone()));
        expr = new_expr;
    }
}
