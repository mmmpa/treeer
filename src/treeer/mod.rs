pub mod child_receiver;
pub mod container_tag;
pub mod core;
pub mod element;
pub mod element_member;
pub mod errors;
pub mod render;
pub mod self_contained_tag;
pub mod state;
pub mod state_attr;
pub mod tag;
pub mod tag_new;
pub mod attr_helpers;

pub mod helpers {
    pub use crate::treeer::attr_helpers::*;
}

pub mod prelude {
    pub use crate::treeer::container_tag::ContainerTag;
    pub use crate::treeer::self_contained_tag::SelfContainedTag;
    pub use crate::treeer::render::Render;
    pub use crate::treeer::element::Workspace;
}
