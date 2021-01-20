use std::collections::{HashMap, HashSet};

use crate::ast::Term;
use crate::ast::Term::*;
use crate::index::bare::BareIdent;
use crate::index::uid::UID;

impl From<Term<UID>> for Term<BareIdent> {
    fn from(term: Term<UID>) -> Self {
        fn _to_bare(
            term: Term<UID>,
            var_maps: &mut HashMap<UID, String>,
            var_set: &mut HashSet<BareIdent>,
        ) -> Term<BareIdent> {
            match term {
                Var(x) => Var(if !var_maps.contains_key(&x) {
                    x.name
                } else {
                    var_maps[&x].clone()
                }),
                Abs(x, e) => {
                    // TODO avoid unnecessary renames (eg. λx1.λx.x1 x instead of λx1.λx2.x1 x2)
                    let bound_name = if e.has_name_collision(&x) {
                        let next_fv_name = e.new_fv_name(&x.name, var_set);
                        var_maps.insert(x, next_fv_name.clone());
                        var_set.insert(next_fv_name.clone());
                        next_fv_name
                    } else {
                        x.name
                    };
                    Abs(bound_name, box _to_bare(*e, var_maps, var_set))
                }
                App(e1, e2) => App(
                    box _to_bare(*e1, var_maps, var_set),
                    box _to_bare(*e2, var_maps, var_set),
                ),
            }
        }
        _to_bare(term, &mut HashMap::default(), &mut HashSet::default())
    }
}
