use crate::treeer::child_receiver::ChildReceiver;
use crate::treeer::core::StateAccessor;
use crate::treeer::state::{StateChild};

/// This is for tags which can contain other tags.
pub trait ContainerTag: StateAccessor + Sized {
    fn append(&mut self, children: Vec<StateChild>) {
        self.with_mut(|core| { core.append(children); Ok(()) }).unwrap();
    }

    fn inc<F>(mut self, f: F) -> Self where Self: Sized + 'static, F: FnOnce(&mut ChildReceiver) {
        let mut tc = ChildReceiver::new();
        f(&mut tc);
        self.append(tc.0);
        self
    }
}
