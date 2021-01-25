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
        _ => unreachable!(),
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
        Rule::app => parse_expr(pair.into_inner().next().unwrap().into_inner()),
        Rule::abs => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let ty = pair.next().unwrap().into_inner().next().unwrap();
            let expr = pair.next().unwrap();
            Abs(
                ident.as_str().to_string(),
                box parse_expr(ty.into_inner()),
                box parse_expr(expr.into_inner()),
            )
        }
        Rule::pi => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let lty = pair.next().unwrap().into_inner().next().unwrap();
            let rty = pair.next().unwrap().into_inner().next().unwrap();
            Pi(
                ident.as_str().to_string(),
                box parse_expr(lty.into_inner()),
                box parse_expr(rty.into_inner()),
            )
        }
        Rule::kind => Term::Kind(match pair.as_str() {
            "*" => Kinds::Star,
            "□" => Kinds::Box,
            "[]" => Kinds::Box,
            _ => unreachable!(),
        }),
        _ => unreachable!(),
    }
}
