use std::fmt::{Debug, Display, Formatter, Result};

pub use Term::*;

pub trait Fresh {
    fn fresh(&self) -> Self;
}

pub trait IdentType: Debug + Display + Clone {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ReduceStrategy {
    CBN,
    NOR,
    CBV,
    APP,
    HAP,
    HSR,
    HNO,
}

pub trait Reducible {
    type ID: IdentType;
    fn subst(&self, ex: &Self) -> Self;
    fn beta_reduce(&self, strategy: ReduceStrategy, limit: Option<usize>) -> Self;
}

#[derive(Clone, Eq, PartialEq)]
pub enum Term<T: IdentType> {
    Var(T),
    Abs(T, Box<Term<T>>),
    App(Box<Term<T>>, Box<Term<T>>),
}

pub fn abs<T: IdentType>(bound: T, term: Term<T>) -> Term<T> {
    Abs(bound, box term)
}

pub fn app<T: IdentType>(lhs: Term<T>, rhs: Term<T>) -> Term<T> {
    App(box lhs, box rhs)
}

impl<T: IdentType> Term<T> {
    fn shows_prec(&self, prec: usize, debug: bool) -> String {
        match self {
            Term::Var(chr) => format!("{}", chr),
            Term::Abs(bound, term) => {
                let rtn = format!("λ{}.{}", bound, term.shows_prec(0, debug));
                if prec > 0 {
                    if debug {
                        format!("({:?})", rtn)
                    } else {
                        format!("({})", rtn)
                    }
                } else {
                    rtn
                }
            }
            Term::App(lhs, rhs) => {
                let rtn = format!("{} {}", lhs.shows_prec(1, debug), rhs.shows_prec(2, debug));
                if prec > 1 {
                    if debug {
                        format!("({:?})", rtn)
                    } else {
                        format!("({})", rtn)
                    }
                } else {
                    rtn
                }
            }
        }
    }
}

impl<T: IdentType> Display for Term<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.shows_prec(0, false))
    }
}

impl<T: IdentType> Debug for Term<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.shows_prec(0, true))
    }
}
