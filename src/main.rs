#![feature(box_syntax)]
#![feature(iterator_fold_self, type_alias_impl_trait)]
#[macro_use]
extern crate pest_derive;

use index::uid::*;

use crate::ast::{ReduceStrategy, Reducible, Term};
use crate::ast::ReduceStrategy::*;
use crate::index::bare::BareIdent;
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
    test_reduce("(\\y.\\x.z x y) x", APP);
    test_reduce("(\\y.\\x.z x y) x", CBN);
    test_reduce("(\\f.\\x.f x) (\\f.\\x.f x)", NOR);
    assert_eq!(
        Term::<UID>::from(parse("(λf.(λx.f (x x)) (λx.f (x x))) \\f.x").unwrap()).equals(&Term::<UID>::from(
            parse("x").unwrap()
        )), true
    )
}

fn test_reduce(expr: &str, strategy: ReduceStrategy) {
    let expr = parse(expr).unwrap();
    let mut expr: Term<UID> = Term::from(expr);
    println!(
        "\n{} => {}",
        Term::<BareIdent>::from(expr.clone()),
        Term::<BareIdent>::from(expr.beta_reduce(strategy, None))
    );
    loop {
        let new_expr = expr.beta_reduce(strategy, Some(1));
        if expr == new_expr {
            break;
        }
        println!("--> {}", Term::<BareIdent>::from(new_expr.clone()));
        expr = new_expr;
    }
}
