use crate::ast::*;
use std::collections::HashMap;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct UID {
    pub name: char,
    pub uid: usize
}

impl IDType for UID {
    fn get_name(&self) -> char {
        self.name
    }
}

pub fn to_uid(term: &Term<BareID>, uid_generator: &mut UIDGenerator) -> Term<UID> {
    _to_uid(term, uid_generator, HashMap::default(), HashMap::default()).0
}

fn _to_uid(
    term: &Term<BareID>,
    uid_generator: &mut UIDGenerator,
    mut free_vars: HashMap<char, usize>,
    mut bound_vars: HashMap<char, usize>,
) -> (Term<UID>, HashMap<char, usize>) {
    match term {
        Var(chr) => {
            let uid = if free_vars.contains_key(chr) {
                free_vars[chr]
            } else if bound_vars.contains_key(chr) {
                bound_vars[chr]
            } else {
                free_vars.insert(*chr, uid_generator.next());
                free_vars[chr]
            };
            (Var(UID { name: *chr, uid }), free_vars)
        }
        Abs(bound, term) => {
            let bound_id = uid_generator.next();
            free_vars.insert(*bound, bound_id);
            Abs(UID{name: *bound, uid: bound_id}, box _to_uid(term, uid_generator, free_vars))
        },
        App(lhs, rhs) => App(box _to_uid(lhs, uid_generator, free_vars.clone()), box _to_uid(rhs, uid_generator, free_vars))
    }
}