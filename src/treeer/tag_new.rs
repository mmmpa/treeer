use modifier::Modifier;

use crate::treeer::element::{OpeningElement, SelfContainedElement};
use crate::treeer::element_member::ContainerElementMember::*;
use crate::treeer::element_member::SelfContainedElementMember::*;
use crate::treeer::element_member::{ContainerElementMember, SelfContainedElementMember};
use crate::treeer::state::State;
use crate::treeer::tag::Tag;

fn new_opening_tag<M: Modifier<State>>(el: ContainerElementMember, m: M) -> OpeningElement {
    OpeningElement::new(el).set(m)
}

fn new_self_contained_tag<M: Modifier<State>>(el: SelfContainedElementMember, m: M) -> SelfContainedElement {
    SelfContainedElement::new(el).set(m)
}

pub struct TagNew;

macro_rules! define_opening_elements {
    ( $( $name:tt => $element:ident ),* ) => { $(
        pub fn $name<M: Modifier<State>>(m: M) -> OpeningElement { new_opening_tag($element, m) }
    )* };
}

macro_rules! define_self_contained_elements {
    ( $( $name:tt => $element:ident ),* ) => { $(
        pub fn $name<M: Modifier<State>>(m: M) -> SelfContainedElement { new_self_contained_tag($element, m) }
    )* };
}

impl TagNew {
    define_opening_elements!(
        a => A,
        abbr => Abbr,
        acronym => Acronym,
        address => Address,
        applet => Applet,
        b => B,
        bdo => Bdo,
        big => Big,
        blockquote => Blockquote,
        body => Body,
        button => Button,
        caption => Caption,
        center => Center,
        cite => Cite,
        code => Code,
        col_group => ColGroup,
        dd => Dd,
        del => Del,
        dfn => Dfn,
        dir => Dir,
        div => Div,
        dl => Dl,
        dt => Dt,
        em => Em,
        fieldset => Fieldset,
        font => Font,
        form => Form,
        frameset => Frameset,
        h1 => H1,
        h2 => H2,
        h3 => H3,
        h4 => H4,
        h5 => H5,
        h6 => H6,
        head => Head,
        html => Html,
        i => I,
        iframe => Iframe,
        ins => Ins,
        isindex => Isindex,
        kbd => Kbd,
        label => Label,
        legend => Legend,
        li => Li,
        map => Map,
        menu => Menu,
        noframes => Noframes,
        noscript => Noscript,
        object => Object,
        ol => Ol,
        opt_group => OptGroup,
        option => Option,
        p => P,
        pre => Pre,
        q => Q,
        s => S,
        samp => Samp,
        script => Script,
        select => Select,
        small => Small,
        span => Span,
        strike => Strike,
        strong => Strong,
        style => Style,
        sub => Sub,
        sup => Sup,
        table => Table,
        tbody => Tbody,
        td => Td,
        textarea => Textarea,
        tfoot => Tfoot,
        th => Th,
        thead => Thead,
        title => Title,
        tr => Tr,
        tt => Tt,
        u => U,
        ul => Ul,
        var => Var
    );

    define_self_contained_elements!(
        area => Area,
        base => Base,
        base_font => BaseFont,
        br => Br,
        col => Col,
        frame => Frame,
        hr => Hr,
        img => Img,
        input => Input,
        link => Link,
        meta => Meta,
        param => Param
    );
}
