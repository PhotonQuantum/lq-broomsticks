pub(crate) mod from;
mod into;
mod reduce;

use std::fmt::{self, Display, Formatter};

use crate::ast::*;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UID {
    pub name: char,
    pub uid: usize,
}

impl Display for UID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.uid)
    }
}

impl IDType for UID {}
