use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.uid)
    }
}

impl IDType for UID {}

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

impl ToBareTerm for Term<UID> {
    fn to_bare(&self) -> Term<BareID> {
        fn _to_bare(term: &Term<UID>, mut var_maps: HashMap<UID, char>) -> Term<BareID> {
            match term {
                Var(x) => Var(if !var_maps.contains_key(x) {
                    x.name
                } else {
                    var_maps[x]
                }),
                Abs(x, e) => {
                    let bound_name = if e.has_name_collision(x) {
                        let last_name = var_maps
                            .values()
                            .fold_first(|x, y| max(x, y))
                            .and_then(|i| Some(*i))
                            .unwrap_or(('a' as u8 - 1) as char);
                        let next_fv_name = e
                            .next_free_var_name((last_name as u8 + 1) as char)
                            .expect("name exhausted.");
                        var_maps.insert(*x, next_fv_name);
                        next_fv_name
                    } else {
                        x.name
                    };
                    Abs(bound_name, box _to_bare(e, var_maps))
                }
                App(e1, e2) => App(
                    box _to_bare(e1, var_maps.clone()),
                    box _to_bare(e2, var_maps),
                ),
            }
        }
        _to_bare(self, HashMap::default())
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
