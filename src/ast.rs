use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;

pub use Term::*;

pub trait Fresh {
    fn fresh(&self) -> Self;
}

pub trait IdentType: Debug + Display + Clone + Eq + Hash {}

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

pub trait Reducible: Sized {
    fn subst(&self, ex: &Self) -> Self;
    fn beta_reduce(&self, strategy: ReduceStrategy, limit: Option<usize>) -> Self;
    fn nf(&self) -> Self {
        self.beta_reduce(ReduceStrategy::HAP, None)
    }
    fn whnf(&self) -> Self {
        self.beta_reduce(ReduceStrategy::CBN, None)
    }
    fn wnf(&self) -> Self {
        self.beta_reduce(ReduceStrategy::CBV, None)
    }
    fn hnf(&self) -> Self {
        self.beta_reduce(ReduceStrategy::HSR, None)
    }
    fn equals(&self, other: &Self) -> bool;
}

type Ty<T> = Term<T>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kinds {
    Star,
    Box,
}

impl Display for Kinds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", if let Kinds::Star = self { "*" } else { "□" })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Term<T: IdentType> {
    Var(T),
    App(Box<Term<T>>, Box<Term<T>>),
    Abs(T, Box<Ty<T>>, Box<Term<T>>),
    Pi(T, Box<Ty<T>>, Box<Ty<T>>),
    Kind(Kinds),
}

pub fn abs<T: IdentType>(bound: T, ty: Ty<T>, term: Term<T>) -> Term<T> {
    Abs(bound, box ty, box term)
}

pub fn app<T: IdentType>(lhs: Term<T>, rhs: Term<T>) -> Term<T> {
    App(box lhs, box rhs)
}

impl<T: IdentType> Term<T> {
    pub fn fv(&self) -> HashSet<&T> {
        match self {
            Var(x) => {
                hashset! {x}
            }
            App(lhs, rhs) => lhs.fv().union(&rhs.fv()).cloned().collect(),
            Abs(x, ty, term) => {
                let mut e_fvs = term.fv();
                e_fvs.remove(x);
                e_fvs.union(&ty.fv()).cloned().collect()
            }
            Pi(x, lty, rty) => {
                let mut e_fvs = rty.fv();
                e_fvs.remove(x);
                e_fvs.union(&lty.fv()).cloned().collect()
            }
            Kind(_) => HashSet::new(),
        }
    }

    fn shows_prec(&self, prec: usize, debug: bool) -> String {
        match self {
            Term::Var(chr) => format!("{}", chr),
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
            Term::Abs(bound, ty, term) => {
                let rtn = format!(
                    "λ{}:{}.{}",
                    bound,
                    ty.shows_prec(0, debug),
                    term.shows_prec(0, debug)
                );
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
            Term::Pi(bound, lty, rty) => {
                let rtn = format!(
                    "π{}:{}.{}",
                    bound,
                    lty.shows_prec(0, debug),
                    rty.shows_prec(0, debug)
                );
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
            Term::Kind(kinds) => kinds.to_string(),
        }
    }
}

impl<T: IdentType> Display for Term<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.shows_prec(0, false))
    }
}

// impl<T: IdentType> Debug for Term<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{}", self.shows_prec(0, true))
//     }
// }
