use std::fmt::{Debug, Display, Formatter, Result};

pub use Term::*;

pub trait IDType: Debug + Display {
    fn get_name(&self) -> char;
}

pub type BareID = char;

impl IDType for BareID {
    fn get_name(&self) -> char {
        *self
    }
}

pub trait ToBareTerm {
    fn to_bare(&self) -> Term<BareID>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term<T: IDType> {
    Var(T),
    Abs(T, Box<Term<T>>),
    App(Box<Term<T>>, Box<Term<T>>),
}

pub fn abs<T: IDType>(bound: T, term: Term<T>) -> Term<T> {
    Abs(bound, box term)
}

pub fn app<T: IDType>(lhs: Term<T>, rhs: Term<T>) -> Term<T> {
    App(box lhs, box rhs)
}

impl<T: IDType> Term<T> {
    fn shows_prec(&self, prec: usize) -> String {
        match self {
            Term::Var(chr) => format!("{}", chr),
            Term::Abs(bound, term) => {
                let rtn = format!("λ{}.{}", bound.get_name(), term.shows_prec(0));
                if prec > 0 { format!("({})", rtn) } else { rtn }
            }
            Term::App(lhs, rhs) => {
                let rtn = format!("{} {}", lhs.shows_prec(1), rhs.shows_prec(2));
                if prec > 1 { format!("({})", rtn) } else { rtn }
            }
        }
    }
}

impl<T: IDType> Display for Term<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.shows_prec(0))
    }
}
