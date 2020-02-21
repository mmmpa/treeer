use crate::treeer::state_attr::{Class, AdhocAttr, IntAttr, IntPerAttr, FlagAttr, PresetAttr};

pub fn class<'a>(v: &'a [&'a str]) -> Class { Class(v) }

pub fn any<'a>(k: &'a str, v: &'a str) -> AdhocAttr<'a> { AdhocAttr(k, v) }

pub fn width(v: usize) -> IntAttr { IntAttr("width", v) }
pub fn height(v: usize) -> IntAttr { IntAttr("height", v) }
pub fn width_per(v: usize) -> IntPerAttr { IntPerAttr("width", v) }
pub fn height_per(v: usize) -> IntPerAttr { IntPerAttr("height", v) }

pub fn disabled() -> FlagAttr { FlagAttr("disabled") }
pub fn checked() -> FlagAttr { FlagAttr("checked") }

macro_rules! define_string_attr {
    ( $( $name:tt => $str:expr ),* ) => { $(
        pub fn $name(v: &str) -> PresetAttr { PresetAttr($str, v) }
    )* };
}

define_string_attr!(
    abbr => "abbr",
    accept => "accept",
    accept_charset => "accept-charset",
    accesskey => "accesskey",
    action => "action",
    align => "align",
    alink => "alink",
    alt => "alt",
    archive => "archive",
    axis => "axis",
    background => "background",
    bgcolor => "bgcolor",
    border => "border",
    cell_padding => "cellpadding",
    cell_spacing => "cellspacing",
    char => "char",
    char_off => "charoff",
    charset => "charset",
    cite => "cite",
    class_id => "classid",
    clear => "clear",
    code => "code",
    codebase => "codebase",
    code_type => "codetype",
    color => "color",
    cols => "cols",
    colspan => "colspan",
    compact => "compact",
    content => "content",
    coords => "coords",
    data => "data",
    datetime => "datetime",
    declare => "declare",
    defer => "defer",
    dir => "dir",
    enctype => "enctype",
    face => "face",
    for_attribute => "for",
    frame => "frame",
    frame_border => "frameborder",
    headers => "headers",
    href => "href",
    hreflang => "hreflang",
    hspace => "hspace",
    http_equiv => "http-equiv",
    id => "id",
    is_map => "ismap",
    label => "label",
    lang => "lang",
    language => "language",
    link => "link",
    long_desc => "longdesc",
    margin_height => "marginheight",
    margin_width => "marginwidth",
    maxlength => "maxlength",
    media => "media",
    method => "method",
    multiple => "multiple",
    name => "name",
    no_href => "nohref",
    no_resize => "noresize",
    no_shade => "noshade",
    nowrap => "nowrap",
    object => "object",
    on_blur => "onblur",
    on_change => "onchange",
    on_click => "onclick",
    on_dblclick => "ondblclick",
    on_focus => "onfocus",
    on_keydown => "onkeydown",
    on_keypress => "onkeypress",
    on_keyup => "onkeyup",
    on_load => "onload",
    on_mousedown => "onmousedown",
    on_mousemove => "onmousemove",
    on_mouseout => "onmouseout",
    on_mouseover => "onmouseover",
    on_mouseup => "onmouseup",
    on_reset => "onreset",
    on_select => "onselect",
    on_submit => "onsubmit",
    on_unload => "onunload",
    profile => "profile",
    prompt => "prompt",
    readonly => "readonly",
    rel => "rel",
    rev => "rev",
    rows => "rows",
    rowspan => "rowspan",
    rules => "rules",
    scheme => "scheme",
    scope => "scope",
    scrolling => "scrolling",
    selected => "selected",
    shape => "shape",
    size => "size",
    span => "span",
    src => "src",
    standby => "standby",
    start => "start",
    style => "style",
    summary => "summary",
    tabindex => "tabindex",
    target => "target",
    text => "text",
    title => "title",
    type_attribute => "type",
    use_map => "usemap",
    valign => "valign",
    value => "value",
    value_type => "valuetype",
    version => "version",
    vlink => "vlink",
    vspace => "vspace"
);
