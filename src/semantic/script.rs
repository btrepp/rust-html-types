use crate::{
    javascript::Javascript,
    node::{Element, Node},
    text::Text,
    url::Url,
};

pub enum Script {
    External(Url),
    Inline(Javascript),
}

impl<'a> From<Script> for Node<'a> {
    fn from(value: Script) -> Self {
        match value {
            Script::External(url) => {
                let el = Element::external_script(url);
                el.into()
            }
            Script::Inline(string) => {
                let el = Element::inline_script(Text::create(string));
                el.into()
            }
        }
    }
}
