use crate::treeer::container_tag::ContainerTag;
use crate::treeer::tag::{Tag, AsTagName};
use crate::treeer::core::{Core, StateAccessor, WithChildren, WithAttr, WithFlag};
use crate::treeer::render::Render;

use std::io::Write;
use crate::treeer::self_contained_tag::SelfContainedTag;
use crate::treeer::element_member::{ContainerElementMember, SelfContainedElementMember};
use crate::treeer::errors::TreeerErr;

#[derive(Debug, Clone)]
pub struct Workspace(Core);

#[derive(Debug, Clone)]
pub struct TextNode(String);

#[derive(Debug, Clone)]
pub struct OpeningElement {
    core: Core,
    element: ContainerElementMember,
}

#[derive(Debug, Clone)]
pub struct SelfContainedElement {
    core: Core,
    element: SelfContainedElementMember,
}

impl Tag for OpeningElement {}
impl Tag for SelfContainedElement {}

impl ContainerTag for Workspace {}
impl ContainerTag for OpeningElement {}

impl SelfContainedTag for SelfContainedElement {}

impl AsTagName for Workspace { fn as_tag_name(&self) -> &'static str { "context" } }
impl AsTagName for OpeningElement { fn as_tag_name(&self) -> &'static str { self.element.name() } }
impl AsTagName for SelfContainedElement { fn as_tag_name(&self) -> &'static str { self.element.name() } }

impl StateAccessor for Workspace {
    fn ref_s(&self) -> &Core { &self.0 }
    fn mut_s(&mut self) -> &mut Core { &mut self.0 }
}
impl StateAccessor for OpeningElement {
    fn ref_s(&self) -> &Core { &self.core }
    fn mut_s(&mut self) -> &mut Core { &mut self.core }
}
impl StateAccessor for SelfContainedElement {
    fn ref_s(&self) -> &Core { &self.core }
    fn mut_s(&mut self) -> &mut Core { &mut self.core }
}

impl Default for Workspace { fn default() -> Self { Self(Core::default()) } }
impl Default for OpeningElement { fn default() -> Self { Self { core: Core::default(), element: ContainerElementMember::Div, } } }
impl Default for SelfContainedElement { fn default() -> Self { Self { core: Core::default(), element: SelfContainedElementMember::Br, } } }

impl OpeningElement { pub fn new(element: ContainerElementMember) -> Self { Self { core: Core::default(), element } } }
impl SelfContainedElement { pub fn new(element: SelfContainedElementMember) -> Self { Self { core: Core::default(), element } } }
impl TextNode { pub fn new(t: &str) -> Self { Self(t.into()) } }

impl WithChildren for Workspace {}
impl WithChildren for OpeningElement {}

impl RenderChildren for Workspace {}
impl RenderChildren for OpeningElement {}
trait RenderChildren: WithChildren {
    fn write_children<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        self.with_children(|children| {
            use crate::treeer::state::StateChild::*;

            for c in children {
                match c {
                    OpeningElement(x) => x.write(w)?,
                    SelfContainedElement(x) => x.write(w)?,
                    TextNode(x) => x.write(w)?,
                }
            };
            Ok(())
        })
    }
}

impl WithAttr for OpeningElement {}
impl WithAttr for SelfContainedElement {}
impl WithFlag for OpeningElement {}
impl WithFlag for SelfContainedElement {}

impl RenderAttr for OpeningElement {}
impl RenderAttr for SelfContainedElement {}
trait RenderAttr: WithAttr + WithFlag {
    fn write_attr<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        self.with_attr(|cs| {
            for (k, v) in cs {
                write!(w, " {}={}", k, json::stringify(v.as_str()))?;
            }
            Ok(())
        })?;

        self.with_flag(|cs| {
            for v in cs {
                write!(w, " {}", v)?;
            }
            Ok(())
        })?;

        Ok(())
    }

    fn render_attr_string(&self)  -> Result<String, TreeerErr> {
        let mut s = vec![];
        self.write_attr(&mut s)?;
        let s = String::from_utf8(s)?;
        Ok(s)
    }
}

impl Render for Workspace {
    fn write<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        self.write_children(w)
    }
}
impl Render for OpeningElement {
    fn write<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        let attr = self.render_attr_string()?;
        write!(w, "<{}{}>", self.as_tag_name(), attr)?;
        self.write_children(w)?;
        write!(w, "</{}>", self.as_tag_name())?;

        Ok(())
    }
}
impl Render for SelfContainedElement {
    fn write<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        let attr = self.render_attr_string()?;
        write!(w, "<{}{} />", self.as_tag_name(), attr)?;
        Ok(())
    }
}
impl Render for TextNode {
    fn write<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr> {
        w.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

