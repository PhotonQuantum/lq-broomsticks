use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{self, Debug, Display, Formatter};
use std::iter::FromIterator;

use crate::ast::*;
use crate::index::bare::BareIdent;

pub(crate) mod from;
mod into;
mod reduce;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UIDGenerator {
    count: usize,
}

impl UIDGenerator {
    pub fn default() -> Self {
        UIDGenerator { count: 0 }
    }
    pub fn next(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}

#[derive(Clone, Debug, Hash)]
pub struct UID {
    pub name: String,
    pub uid: usize,
}

impl PartialEq for UID {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for UID {}

impl Display for UID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}:{}}}", self.name, self.uid)
    }
}

impl IdentType for UID {}

impl Term<UID> {
    pub fn new_fv_name(&self, old_name: &BareIdent, var_set: &HashSet<BareIdent>) -> BareIdent {
        let free_var_names = self.fv_names();
        let mut name = old_name.clone();
        while free_var_names.contains(&name) || var_set.contains(&name) {
            name = name.fresh();
        }
        name
    }

    pub fn fv_names(&self) -> HashSet<BareIdent> {
        self.fv().iter().map(|ident| ident.name.clone()).collect()
    }

    pub fn uid_generator(&self) -> UIDGenerator {
        UIDGenerator {
            count: self.fv().iter().fold(0usize, |count, v| max(count, v.uid)),
        }
    }

    pub fn has_name_collision(&self, bound_var: &UID) -> bool {
        match self {
            Var(x) => x != bound_var && x.name == bound_var.name,
            Abs(_, e) => e.has_name_collision(bound_var),
            App(e1, e2) => e1.has_name_collision(bound_var) || e2.has_name_collision(bound_var),
        }
    }
}
