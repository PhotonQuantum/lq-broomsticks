#![feature(box_syntax)]
#![feature(iterator_fold_self, type_alias_impl_trait)]
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate pest_derive;

use index::uid::*;

use crate::ast::ReduceStrategy::*;
use crate::ast::{ReduceStrategy, Reducible, Term};
use crate::index::bare::BareIdent;
use crate::parser::parse;

mod ast;
mod index;
mod parser;

fn main() {
    // !WARN! ill-typed terms
    test_reduce(
        "(λa:*.λb:*.λc:*.a (λd:*.λe:*.e (d b)) (λd:*.c) (λd:*.d)) (λa:*.λb:*.a b)",
        // "(λa.λb.λc.a (λd.λe.e (d b)) (λd.c) (λd.d)) (λa.λb.a b)",
        APP,
    );
    test_reduce(
        "(λa:*.λb:*.λc:*.a (λd:*.λe:*.e (d b)) (λd:*.c) (λd:*.d)) (λa:*.λb:*.a b)",
        CBN,
    );
    test_reduce("(λf:*.(λx:*.f (x x)) (λx:*.f (x x))) λf:*.x", APP);
    // (λf.(λx.f (x x)) (λx.f (x x))) λf.x
    test_reduce("(λf:*.(λx:*.f (x x)) (λx:*.f (x x))) λf:*.x", CBN);
    test_reduce("(λy:*.λx:*.z x y) x", APP);
    // (λy.λx.z x y) x
    test_reduce("(λy:*.λx:*.z x y) x", CBN);
    test_reduce("(λf:*.λx:*.f x) (λf:*.λx:*.f x)", NOR);
    assert_eq!(
        Term::<UID>::from(parse("(λf:*.(λx:*.f (x x)) (λx:*.f (x x))) λf:*.x").unwrap())
            .equals(&Term::<UID>::from(parse("x").unwrap())),
        true
    );
    test_parse("λA:*.λB:*.πC:*.π_:π_:A.C.π_:π_:B.C.C"); // λ A: *. λ B: *. (π C: *. (A → C) → (B → C) → C)
    test_parse("πA:*.πB:*.πC:*.π_:π_:A.B.C"); // forall A B C, (A -> B) -> C
    test_parse("πA:*.πB:*.πC:*.π_:A.π_:B.C"); // forall A B C, A -> B -> C
}

fn test_parse(expr: &str) {
    println!("\n{}", expr);
    println!("{}", parse(expr).unwrap());
    assert_eq!(
        parse(expr).unwrap(),
        parse(parse(expr).unwrap().to_string().as_str()).unwrap()
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
