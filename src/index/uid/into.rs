use crate::ast::Term::*;
use crate::ast::{BareID, Term};
use crate::index::uid::UID;
use std::cmp::max;
use std::collections::HashMap;

impl From<Term<UID>> for Term<BareID> {
    fn from(term: Term<UID>) -> Self {
        fn _to_bare(term: Term<UID>, mut var_maps: HashMap<UID, char>) -> Term<BareID> {
            match term {
                Var(x) => Var(if !var_maps.contains_key(&x) {
                    x.name
                } else {
                    var_maps[&x]
                }),
                Abs(x, e) => {
                    let bound_name = if e.has_name_collision(&x) {
                        let last_name = var_maps
                            .values()
                            .fold_first(|x, y| max(x, y))
                            .and_then(|i| Some(*i))
                            .unwrap_or(('a' as u8 - 1) as char);
                        let next_fv_name = e
                            .next_free_var_name((last_name as u8 + 1) as char)
                            .expect("name exhausted.");
                        var_maps.insert(x, next_fv_name);
                        next_fv_name
                    } else {
                        x.name
                    };
                    Abs(bound_name, box _to_bare(*e, var_maps))
                }
                App(e1, e2) => App(
                    box _to_bare(*e1, var_maps.clone()),
                    box _to_bare(*e2, var_maps),
                ),
            }
        }
        _to_bare(term, HashMap::default())
    }
}
