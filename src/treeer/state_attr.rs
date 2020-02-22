// ref: Index of the HTML 4 Attributes https://www.w3.org/TR/html401/index/attributes.html

pub struct AdhocAttr(pub String, pub String);
pub struct PresetAttr(pub &'static str, pub String);
pub struct IntAttr(pub &'static str, pub usize);
pub struct IntPerAttr(pub &'static str, pub usize);
pub struct FlagAttr(pub &'static str);
pub struct Class<'a>(pub &'a [&'a str]);

#[cfg(test)]
mod tests {
    use modifier::{Modifier};
    use crate::treeer::state_attr::*;
    use crate::treeer::state::State;

    #[test]
    fn it_works() {
        let mut state = State::default();
        AdhocAttr("any".into(), "some".into()).modify(&mut state);
        PresetAttr("preset", "value".into()).modify(&mut state);
        IntAttr("height", 100).modify(&mut state);
        IntPerAttr("width", 200).modify(&mut state);
        Class(&["a", "b"]).modify(&mut state);
        FlagAttr("disable").modify(&mut state);


        assert_eq!(state.attr().get("any").unwrap(), "some");
        assert_eq!(state.attr().get("preset").unwrap(), "value");
        assert_eq!(state.attr().get("height").unwrap(), "100");
        assert_eq!(state.attr().get("width").unwrap(), "200%");
        assert_eq!(state.attr().get("class").unwrap(), "a b");
        assert_eq!(state.flags().get(0).unwrap(), "disable");
    }
}
