use std::fmt::{Debug, Display, Formatter, Result};

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
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
    pub fn fv(&self) -> HashSet<&T> {
        self._fv(HashSet::default())
    }

    fn _fv<'a>(&'a self, mut bound_vars: HashSet<&'a T>) -> HashSet<&'a T> {
        match self {
            Var(x) => {
                if !bound_vars.contains(x) {
                    let mut fv_set = HashSet::default();
                    fv_set.insert(x);
                    fv_set
                } else {
                    HashSet::default()
                }
            }
            Abs(x, e) => {
                bound_vars.insert(x);
                e._fv(bound_vars)
            }
            App(e1, e2) => {
                let lhs_fvs = e1._fv(bound_vars.clone());
                let rhs_fvs = e2._fv(bound_vars);
                HashSet::from_iter(lhs_fvs.union(&rhs_fvs).cloned())
            }
        }
    }

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
