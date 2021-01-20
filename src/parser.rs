use std::result;

use pest::error::Error;
use pest::Parser;

use crate::ast::*;
use crate::index::bare::BareIdent;

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;

pub fn parse(source: &str) -> result::Result<Term<BareIdent>, Error<Rule>> {
    let mut ast = vec![];

    let mut pairs = LambdaParser::parse(Rule::lambda, source)?;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::lambda => {
            let pairs = pair.into_inner();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::expr => ast.push(parse_expr(pair.into_inner())),
                    Rule::EOI => {}
                    _ => unreachable!(),
                }
            }
        }
        _ => {}
    }

    Ok(ast.first().unwrap().clone())
}

fn parse_expr<'a>(pairs: impl Iterator<Item = pest::iterators::Pair<'a, Rule>>) -> Term<BareIdent> {
    pairs
        .into_iter()
        .map(|term| parse_term(term.into_inner().next().unwrap()))
        .fold(None, |pred, term| {
            Some(match pred {
                None => term,
                Some(pred) => App(box pred, box term),
            })
        })
        .unwrap()
}

fn parse_term(pair: pest::iterators::Pair<Rule>) -> Term<BareIdent> {
    match pair.as_rule() {
        Rule::var => Var(pair.as_str().to_string()),
        Rule::abs => {
            let mut pair = pair.into_inner();
            let bound = pair.next().unwrap();
            let expr = pair.next().unwrap();
            Abs(
                bound.as_str().to_string(),
                box parse_expr(expr.into_inner()),
            )
        }
        Rule::app => parse_expr(pair.into_inner().next().unwrap().into_inner()),
        _ => unreachable!(),
    }
}
