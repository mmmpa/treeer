use modifier::Modifier;
use modifier::Set;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::treeer::errors::TreeerErr;
use crate::treeer::state::{State, StateChild, Attr, Flags};

#[derive(Debug, Clone)]
pub struct Core(CoreInner);
pub type CoreInner = Rc<RefCell<State>>;

impl Default for Core {
    fn default() -> Self { Self(Rc::new(RefCell::new(State::default()))) }
}

// Core 以外の State を持つものが出てきたらデフォルト実装を外して OpeningElement などに移動する
pub trait WithChildren: Debug + StateAccessor {
    fn with_children<F: FnOnce(&[StateChild]) -> Result<(), TreeerErr>>(&self, f: F) -> Result<(), TreeerErr> {
        self.with_ref(|core| f(core.children()))
    }
}

pub trait WithAttr: Debug + StateAccessor {
    fn with_attr<F: FnOnce(&Attr) -> Result<(), TreeerErr>>(&self, f: F) -> Result<(), TreeerErr> {
        self.with_ref(|core| f(core.attr()))
    }
}

pub trait WithFlag: Debug + StateAccessor {
    fn with_flag<F: FnOnce(&Flags) -> Result<(), TreeerErr>>(&self, f: F) -> Result<(), TreeerErr> {
        self.with_ref(|core| f(core.flags()))
    }
}

pub trait StateAccessor: Debug {
    fn ref_s(&self) -> &Core;
    fn mut_s(&mut self) -> &mut Core;

    fn with_mut<F: FnOnce(&mut State) -> Result<(), TreeerErr>>(&mut self, f: F) -> Result<(), TreeerErr> {
        match self.mut_s().0.try_borrow_mut() {
            Err(_) => Err(TreeerErr::default()),
            Ok(mut core) => { f(&mut core) },
        }
    }

    fn with_ref<F: FnOnce(&State) -> Result<(), TreeerErr>>(&self, f: F) -> Result<(), TreeerErr> {
        match self.ref_s().0.try_borrow() {
            Err(_) => Err(TreeerErr::default()),
            Ok(core) => { f(&core) },
        }
    }

    fn set_to_core<M: Modifier<State>>(&mut self, m: M) {
        self.with_mut(|core| {
            core.set_mut(m);
            Ok(())
        }).unwrap();
    }
}
