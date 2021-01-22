use crate::ast::*;
use crate::index::bare::BareIdent;
use crate::index::uid::UID;

macro_rules! break_by_limit {
    ($self: ident, $limit: ident) => {
        if let Some(i) = $limit {
            if i == 0 {
                return $self.clone();
            }
        }
    };
}

impl Term<UID> {
    fn _subst(&self, from: &UID, to: &Term<UID>) -> Term<UID> {
        match self {
            Var(x) => {
                if x.uid == from.uid {
                    to.clone()
                } else {
                    self.clone()
                }
            }
            Abs(x, e) => Abs(x.clone(), box e._subst(from, to)),
            App(e1, e2) => App(box e1._subst(from, to), box e2._subst(from, to)),
        }
    }

    fn cbn_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.cbn_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(e2)
                    .cbn_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_, e2.clone()),
            },
            _ => self.clone(),
        }
    }

    fn nor_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            Abs(x, e) => Abs(x.clone(), box e.nor_reduce(limit)),
            App(e1, e2) => match e1.cbn_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(e2)
                    .nor_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_.nor_reduce(limit), box e2.nor_reduce(limit)),
            },
            _ => self.clone(),
        }
    }

    fn cbv_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.cbv_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(&e2.cbv_reduce(limit))
                    .cbv_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_, box e2.cbv_reduce(limit)),
            },
            _ => self.clone(),
        }
    }

    fn app_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.app_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(&e2.app_reduce(limit))
                    .app_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_, box e2.app_reduce(limit)),
            },
            Abs(x, e) => Abs(x.clone(), box e.app_reduce(limit)),
            _ => self.clone(),
        }
    }

    fn hap_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.cbv_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(&e2.hap_reduce(limit))
                    .hap_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_.hap_reduce(limit), box e2.hap_reduce(limit)),
            },
            Abs(x, e) => Abs(x.clone(), box e.hap_reduce(limit)),
            _ => self.clone(),
        }
    }

    fn hsr_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.hsr_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(&e2)
                    .hsr_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_, e2.clone()),
            },
            Abs(x, e) => Abs(x.clone(), box e.hsr_reduce(limit)),
            _ => self.clone(),
        }
    }

    fn hno_reduce(&self, limit: Option<usize>) -> Term<UID> {
        break_by_limit!(self, limit);
        match self {
            App(e1, e2) => match e1.hsr_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(&e2)
                    .hno_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_.hno_reduce(limit), box e2.hno_reduce(limit)),
            },
            Abs(x, e) => Abs(x.clone(), box e.hno_reduce(limit)),
            _ => self.clone(),
        }
    }
}

impl Reducible for Term<UID> {
    fn subst(&self, ex: &Self) -> Self {
        if let Abs(x, e) = self {
            e._subst(x, ex)
        } else {
            panic!("only abstraction can be substituted.")
        }
    }

    fn beta_reduce(&self, strategy: ReduceStrategy, limit: Option<usize>) -> Self {
        let limit = if let Some(l) = limit {
            Some(l)
        } else {
            Some(100)
        };
        match strategy {
            ReduceStrategy::CBN => self.cbn_reduce(limit),
            ReduceStrategy::NOR => self.nor_reduce(limit),
            ReduceStrategy::CBV => self.cbv_reduce(limit),
            ReduceStrategy::APP => self.app_reduce(limit),
            ReduceStrategy::HAP => self.hap_reduce(limit),
            ReduceStrategy::HSR => self.hsr_reduce(limit),
            ReduceStrategy::HNO => self.hno_reduce(limit),
        }
    }

    fn equals(&self, other: &Self) -> bool {
        /// Alpha and eta convertible terms are considered equal.
        fn reassign_uid(term: Term<UID>) -> Term<UID> {
            Term::<UID>::from(Term::<BareIdent>::from(term))
        }

        reassign_uid(
            App(
                box self.clone(),
                box Var(UID {
                    name: String::from("_"),
                    uid: self.uid_generator().next(),
                }),
            )
            .nf(),
        ) == reassign_uid(
            App(
                box other.clone(),
                box Var(UID {
                    name: String::from("_"),
                    uid: other.uid_generator().next(),
                }),
            )
            .nf(),
        )
    }
}
