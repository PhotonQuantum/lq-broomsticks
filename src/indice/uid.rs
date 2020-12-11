use std::cmp::max;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::{Display, Formatter, Result};
use std::iter::FromIterator;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.name, self.uid)
    }
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
            bound_vars.insert(*bound, bound_id);
            let (term, free_vars) = _to_uid(term, uid_generator, free_vars, bound_vars);
            (
                Abs(
                    UID {
                        name: *bound,
                        uid: bound_id,
                    },
                    box term,
                ),
                free_vars,
            )
        }
        App(lhs, rhs) => {
            let (lhs, free_vars) = _to_uid(lhs, uid_generator, free_vars, bound_vars.clone());
            let (rhs, free_vars) = _to_uid(rhs, uid_generator, free_vars, bound_vars);
            (App(box lhs, box rhs), free_vars)
        }
    }
}

impl ToBareTerm for Term<UID> {
    fn to_bare(&self) -> Term<BareID> {
        fn _to_bare(term: &Term<UID>, mut var_maps: HashMap<UID, char>) -> Term<BareID> {
            match term {
                Var(chr) => Var(if !var_maps.contains_key(chr) {
                    chr.get_name()
                } else {
                    var_maps[chr]
                }),
                Abs(bound, term) => {
                    let bound_name = if term.has_name_collision(bound) {
                        let last_name = var_maps
                            .values()
                            .fold_first(|x, y| max(x, y))
                            .and_then(|i| Some(*i))
                            .unwrap_or(('a' as u8 - 1) as char);
                        let next_fv_name = term
                            .next_free_var_name((last_name as u8 + 1) as char)
                            .expect("name exhausted.");
                        var_maps.insert(*bound, next_fv_name);
                        next_fv_name
                    } else {
                        bound.get_name()
                    };
                    Abs(bound_name, box _to_bare(term, var_maps))
                }
                App(lhs, rhs) => App(
                    box _to_bare(lhs, var_maps.clone()),
                    box _to_bare(rhs, var_maps),
                ),
            }
        }
        _to_bare(self, HashMap::default())
    }
}

impl Term<UID> {
    fn next_free_var_name(&self, start_char: char) -> Option<char> {
        let free_var_names = self.free_var_names();
        for chr in start_char..('z' as u8 + 1) as char {
            if !free_var_names.contains(&chr) {
                return Some(chr);
            }
        }
        None
    }

    fn free_var_names(&self) -> HashSet<char> {
        self._free_var_names(HashSet::default())
    }

    fn _free_var_names(&self, mut bound_vars: HashSet<UID>) -> HashSet<char> {
        match self {
            Var(uid) => {
                if !bound_vars.contains(uid) {
                    let mut fvs = HashSet::default();
                    fvs.insert(uid.name);
                    fvs
                } else {
                    HashSet::default()
                }
            }
            Abs(bound, term) => {
                bound_vars.insert(*bound);
                term._free_var_names(bound_vars)
            }
            App(lhs, rhs) => {
                let lhs_fvs = lhs._free_var_names(bound_vars.clone());
                let rhs_fvs = rhs._free_var_names(bound_vars);
                HashSet::from_iter(lhs_fvs.union(&rhs_fvs).copied())
            }
        }
    }

    fn has_name_collision(&self, bound_var: &UID) -> bool {
        match self {
            Var(uid) => uid != bound_var && uid.name == bound_var.name,
            Abs(_, term) => term.has_name_collision(bound_var),
            App(lhs, rhs) => lhs.has_name_collision(bound_var) || rhs.has_name_collision(bound_var),
        }
    }
}
