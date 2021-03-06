// ref: Index of the HTML 4 Elements https://www.w3.org/TR/html401/index/elements.html
#[derive(Debug, Clone)]
pub enum ContainerElementMember {
    A,
    Abbr,
    Acronym,
    Address,
    Applet,
    B,
    Bdo,
    Big,
    Blockquote,
    Body,
    Button,
    Caption,
    Center,
    Cite,
    Code,
    ColGroup,
    Dd,
    Del,
    Dfn,
    Dir,
    Div,
    Dl,
    Dt,
    Em,
    Fieldset,
    Font,
    Form,
    Frameset,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Html,
    I,
    Iframe,
    Ins,
    Isindex,
    Kbd,
    Label,
    Legend,
    Li,
    Map,
    Menu,
    Noframes,
    Noscript,
    Object,
    Ol,
    OptGroup,
    Option,
    P,
    Pre,
    Q,
    S,
    Samp,
    Script,
    Select,
    Small,
    Span,
    Strike,
    Strong,
    Style,
    Sub,
    Sup,
    Table,
    Tbody,
    Td,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Title,
    Tr,
    Tt,
    U,
    Ul,
    Var,
}

#[derive(Debug, Clone)]
pub enum SelfContainedElementMember {
    Area,
    Base,
    BaseFont,
    Br,
    Col,
    Frame,
    Hr,
    Img,
    Input,
    Link,
    Meta,
    Param,
}

impl ContainerElementMember {
    pub fn name(&self) -> &'static str {
        use self::ContainerElementMember::*;

        match self {
            A => "a",
            Abbr => "abbr",
            Acronym => "acronym",
            Address => "address",
            Applet => "applet",
            B => "b",
            Bdo => "bdo",
            Big => "big",
            Blockquote => "blockquote",
            Body => "body",
            Button => "button",
            Caption => "caption",
            Center => "center",
            Cite => "cite",
            Code => "code",
            ColGroup => "colgroup",
            Dd => "dd",
            Del => "del",
            Dfn => "dfn",
            Dir => "dir",
            Div => "div",
            Dl => "dl",
            Dt => "dt",
            Em => "em",
            Fieldset => "fieldset",
            Font => "font",
            Form => "form",
            Frameset => "frameset",
            H1 => "h1",
            H2 => "h2",
            H3 => "h3",
            H4 => "h4",
            H5 => "h5",
            H6 => "h6",
            Head => "head",
            Html => "html",
            I => "i",
            Iframe => "iframe",
            Ins => "ins",
            Isindex => "isindex",
            Kbd => "kbd",
            Label => "label",
            Legend => "legend",
            Li => "li",
            Map => "map",
            Menu => "menu",
            Noframes => "noframes",
            Noscript => "noscript",
            Object => "object",
            Ol => "ol",
            OptGroup => "optgroup",
            Option => "option",
            P => "p",
            Pre => "pre",
            Q => "q",
            S => "s",
            Samp => "samp",
            Script => "script",
            Select => "select",
            Small => "small",
            Span => "span",
            Strike => "strike",
            Strong => "strong",
            Style => "style",
            Sub => "sub",
            Sup => "sup",
            Table => "table",
            Tbody => "tbody",
            Td => "td",
            Textarea => "textarea",
            Tfoot => "tfoot",
            Th => "th",
            Thead => "thead",
            Title => "title",
            Tr => "tr",
            Tt => "tt",
            U => "u",
            Ul => "ul",
            Var => "var",
        }
    }
}

impl SelfContainedElementMember {
    pub fn name(&self) -> &'static str {
        use self::SelfContainedElementMember::*;

        match self {
            Area => "area",
            Base => "base",
            BaseFont => "basefont",
            Br => "br",
            Col => "col",
            Frame => "frame",
            Hr => "hr",
            Img => "img",
            Input => "input",
            Link => "link",
            Meta => "meta",
            Param => "param",
        }
    }
}
