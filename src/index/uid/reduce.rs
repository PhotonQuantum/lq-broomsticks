use crate::ast::*;
use crate::index::uid::UID;

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
            Abs(x, e) => Abs(*x, box e._subst(from, to)),
            App(e1, e2) => App(box e1._subst(from, to), box e2._subst(from, to)),
        }
    }

    fn cbn_reduce(&self, limit: Option<usize>) -> Term<UID> {
        if let Some(i) = limit {
            if i == 0 {
                return self.clone();
            }
        }
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
        if let Some(i) = limit {
            if i == 0 {
                return self.clone();
            }
        }
        match self {
            Abs(x, e) => Abs(*x, box e.nor_reduce(limit)),
            App(e1, e2) => match e1.cbn_reduce(limit) {
                Abs(x, e) => Abs(x, e)
                    .subst(e2)
                    .nor_reduce(limit.and_then(|i| Some(i - 1))),
                e1_ => App(box e1_.nor_reduce(limit), box e2.nor_reduce(limit)),
            },
            _ => self.clone(),
        }
    }
}

impl Reducible for Term<UID> {
    type ID = UID;

    fn subst(&self, ex: &Self) -> Self {
        if let Abs(x, e) = self {
            e._subst(x, ex)
        } else {
            panic!("only abstraction can be substituted.")
        }
    }

    fn beta_reduce(&self, strategy: ReduceStrategy, limit: Option<usize>) -> Self {
        match strategy {
            ReduceStrategy::CBN => self.cbn_reduce(limit),
            ReduceStrategy::NOR => self.nor_reduce(limit),
            _ => unimplemented!(),
        }
    }
}
