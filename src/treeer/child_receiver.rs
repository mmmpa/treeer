use modifier::Modifier;

use crate::treeer::element::{SelfContainedElement, OpeningElement};
use crate::treeer::state::{StateChild, State};
use crate::treeer::tag_new::TagNew;

macro_rules! define_opening_elements {
    ( $($element:tt),* ) => { $(
        pub fn $element<M: Modifier<State>>(&mut self, m: M) -> OpeningElement {
            let el = TagNew::$element(m);
            self.push(StateChild::OpeningElement(el.clone()));
            el
        }
    )* };
    ( $($element:tt),+ , ) => { define_opening_elements!($( $element ),*); };
}

macro_rules! define_self_contained_elements {
    ( $($element:tt),* ) => { $(
        pub fn $element<M: Modifier<State>>(&mut self, m: M) -> SelfContainedElement {
            let el = TagNew::$element(m);
            self.push(StateChild::SelfContainedElement(el.clone()));
            el
        }
    )* };
    ( $($element:tt),+ , ) => { define_self_contained_elements!($( $element ),*); };
}

/// This contains new tags for ContainerTag temporarily.
pub struct ChildReceiver(pub Vec<StateChild>);

impl Default for ChildReceiver {
    fn default() -> Self {
        Self(vec![])
    }
}

impl ChildReceiver {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, child: StateChild) {
        self.0.push(child);
    }

    define_opening_elements!(
        a,
        abbr,
        acronym,
        address,
        applet,
        b,
        bdo,
        big,
        blockquote,
        body,
        button,
        caption,
        center,
        cite,
        code,
        col_group,
        dd,
        del,
        dfn,
        dir,
        div,
        dl,
        dt,
        em,
        fieldset,
        font,
        form,
        frameset,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        head,
        html,
        i,
        iframe,
        ins,
        isindex,
        kbd,
        label,
        legend,
        li,
        map,
        menu,
        noframes,
        noscript,
        object,
        ol,
        opt_group,
        option,
        p,
        pre,
        q,
        s,
        samp,
        script,
        select,
        small,
        span,
        strike,
        strong,
        style,
        sub,
        sup,
        table,
        tbody,
        td,
        textarea,
        tfoot,
        th,
        thead,
        title,
        tr,
        tt,
        u,
        ul,
        var,
    );

    define_self_contained_elements!(
        area,
        base,
        base_font,
        br,
        col,
        frame,
        hr,
        img,
        input,
        link,
        meta,
        param,
    );
}
