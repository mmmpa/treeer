use modifier::{Modifier, Set};
use std::collections::BTreeMap;
use crate::treeer::state_attr::*;

use crate::treeer::element::{TextNode, SelfContainedElement, OpeningElement};
use std::fmt::Debug;


pub type Attr = BTreeMap<String, String>;
pub type Flags = Vec<String>;

#[derive(Debug)]
pub struct State {
    _attr: Attr,
    _children: Vec<StateChild>,
    _flags: Flags,
}

#[derive(Debug)]
pub enum StateChild {
    OpeningElement(OpeningElement),
    SelfContainedElement(SelfContainedElement),
    TextNode(TextNode),
}

impl State {
    pub fn default() -> Self {
        Self {
            _attr: BTreeMap::new(),
            _children: vec![],
            _flags: vec![],
        }
    }

    pub fn push(&mut self, child: StateChild) {
        self._children.push(child);
    }

    pub fn append(&mut self, mut children: Vec<StateChild>) {
        self._children.append(&mut children);
    }


    pub fn children(&self) -> &[StateChild] {
        &self._children
    }

    pub fn attr(&self) -> &Attr {
        &self._attr
    }

    pub fn flags(&self) -> &Flags {
        &self._flags
    }
}

impl Set for State {}

impl Modifier<State> for () {
    fn modify(self, _src: &mut State) {}
}

impl Modifier<State> for AdhocAttr<'_> {
    fn modify(self, src: &mut State) {
        let AdhocAttr(key, value) = self;
        src._attr.insert(key.into(), value.into());
    }
}

impl Modifier<State> for PresetAttr<'_> {
    fn modify(self, src: &mut State) {
        let PresetAttr(key, value) = self;
        src._attr.insert(key.into(), value.into());
    }
}

impl Modifier<State> for IntAttr {
    fn modify(self, src: &mut State) {
        let IntAttr(key, value) = self;
        src._attr.insert(key.into(), value.to_string());
    }
}

impl Modifier<State> for IntPerAttr {
    fn modify(self, src: &mut State) {
        let IntPerAttr(key, value) = self;
        src._attr.insert(key.into(), format!("{}%", value));
    }
}

impl Modifier<State> for FlagAttr {
    fn modify(self, src: &mut State) {
        let FlagAttr(key) = self;
        src._flags.push(key.into());
    }
}

impl Modifier<State> for Class<'_> {
    fn modify(self, src: &mut State) {
        src._attr.insert("class".into(), self.0.join(" "));
    }
}

impl Modifier<State> for &str {
    fn modify(self, src: &mut State) {
        src.push(StateChild::TextNode(TextNode::new(self)));
    }
}
