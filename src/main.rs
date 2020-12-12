#![feature(box_syntax)]
#![feature(iterator_fold_self, type_alias_impl_trait)]
#[macro_use]
extern crate pest_derive;

use crate::ast::ReduceStrategy::NOR;
use crate::ast::{BareID, IDType, Reducible, Term};
use crate::parser::parse;

use index::uid::*;

mod ast;
mod index;
mod parser;

fn main() {
    test_reduce("(λa.λb.λc.a (λd.λe.e (d b)) (λd.c) (λd.d)) (λa.λb.a b)");
    test_reduce("(λf.(λx.f (x x)) (λx.f (x x))) \\f.x");
    test_reduce("(\\y.\\x.yxz)x");
}

fn test_reduce(expr: &str) {
    let expr = parse(expr).unwrap();
    let mut expr = from::to_uid(&expr, &mut UIDGenerator::default());
    println!("{}", Term::<BareID>::from(expr.clone()));
    loop {
        let new_expr = expr.beta_reduce(NOR, Some(1));
        if expr == new_expr {
            break;
        }
        println!("--> {}", Term::<BareID>::from(new_expr.clone()));
        expr = new_expr;
    }
}
