use std::collections::HashMap;

use crate::ast::Term::*;
use crate::ast::{Kinds, Term};
use crate::index::bare::BareIdent;
use crate::index::uid::{UIDGenerator, UID};

impl From<Term<BareIdent>> for Term<UID> {
    fn from(term: Term<BareIdent>) -> Self {
        from_bare(&term, &mut UIDGenerator::default())
    }
}

pub fn from_bare(term: &Term<BareIdent>, uid_generator: &mut UIDGenerator) -> Term<UID> {
    _from_bare(term, uid_generator, HashMap::default(), HashMap::default()).0
}

fn _from_bare(
    term: &Term<BareIdent>,
    uid_generator: &mut UIDGenerator,
    mut free_vars: HashMap<BareIdent, usize>,
    mut bound_vars: HashMap<BareIdent, usize>,
) -> (Term<UID>, HashMap<BareIdent, usize>) {
    match term {
        App(e1, e2) => {
            let (lhs, free_vars) = _from_bare(e1, uid_generator, free_vars, bound_vars.clone());
            let (rhs, free_vars) = _from_bare(e2, uid_generator, free_vars, bound_vars);
            (App(box lhs, box rhs), free_vars)
        }
        Var(x) => {
            let uid = if free_vars.contains_key(x) {
                free_vars[x]
            } else if bound_vars.contains_key(x) {
                bound_vars[x]
            } else {
                free_vars.insert(x.clone(), uid_generator.next());
                free_vars[x]
            };
            (
                Var(UID {
                    name: x.clone(),
                    uid,
                }),
                free_vars,
            )
        }
        Abs(x, _, e) => {
            let bound_id = uid_generator.next();
            bound_vars.insert(x.clone(), bound_id);
            let (term, free_vars) = _from_bare(e, uid_generator, free_vars, bound_vars);
            (
                Abs(
                    UID {
                        name: x.clone(),
                        uid: bound_id,
                    },
                    box Term::Kind(Kinds::Star), // TODO
                    box term,
                ),
                free_vars,
            )
        }
        Pi(_, _, _) => unimplemented!(), // TODO
        Kind(kind) => (Term::Kind(*kind), free_vars),
    }
}
