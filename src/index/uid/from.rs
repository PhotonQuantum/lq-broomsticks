use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::ast::*;
use crate::index::uid::{UIDGenerator, UID};

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
        Var(x) => {
            let uid = if free_vars.contains_key(x) {
                free_vars[x]
            } else if bound_vars.contains_key(x) {
                bound_vars[x]
            } else {
                free_vars.insert(*x, uid_generator.next());
                free_vars[x]
            };
            (Var(UID { name: *x, uid }), free_vars)
        }
        Abs(x, e) => {
            let bound_id = uid_generator.next();
            bound_vars.insert(*x, bound_id);
            let (term, free_vars) = _to_uid(e, uid_generator, free_vars, bound_vars);
            (
                Abs(
                    UID {
                        name: *x,
                        uid: bound_id,
                    },
                    box term,
                ),
                free_vars,
            )
        }
        App(e1, e2) => {
            let (lhs, free_vars) = _to_uid(e1, uid_generator, free_vars, bound_vars.clone());
            let (rhs, free_vars) = _to_uid(e2, uid_generator, free_vars, bound_vars);
            (App(box lhs, box rhs), free_vars)
        }
    }
}

impl Term<UID> {
    pub fn next_free_var_name(&self, start_char: char) -> Option<char> {
        let free_var_names = self.free_var_names();
        for chr in start_char..('z' as u8 + 1) as char {
            if !free_var_names.contains(&chr) {
                return Some(chr);
            }
        }
        None
    }

    pub fn free_var_names(&self) -> HashSet<char> {
        self._free_var_names(HashSet::default())
    }

    fn _free_var_names(&self, mut bound_vars: HashSet<UID>) -> HashSet<char> {
        match self {
            Var(x) => {
                if !bound_vars.contains(x) {
                    let mut fvs = HashSet::default();
                    fvs.insert(x.name);
                    fvs
                } else {
                    HashSet::default()
                }
            }
            Abs(x, e) => {
                bound_vars.insert(*x);
                e._free_var_names(bound_vars)
            }
            App(e1, e2) => {
                let lhs_fvs = e1._free_var_names(bound_vars.clone());
                let rhs_fvs = e2._free_var_names(bound_vars);
                HashSet::from_iter(lhs_fvs.union(&rhs_fvs).copied())
            }
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
