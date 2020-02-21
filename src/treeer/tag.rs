use modifier::Modifier;
use std::fmt::Debug;
use crate::treeer::state::{State};
use crate::treeer::core::StateAccessor;


pub trait AsTagName { fn as_tag_name(&self) -> &'static str; }

pub trait Tag: Debug + 'static + StateAccessor + Sized {
    fn set<M: Modifier<State>>(mut self, m: M) -> Self {
        self.set_to_core(m);
        self
    }

    fn set_mut<M: Modifier<State>>(&mut self, m: M) {
        self.set_to_core(m);
    }
}
