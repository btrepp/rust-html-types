use crate::{
    css::CascadingStyleSheet,
    node::{Element, Node},
    text::Text,
    url::Url,
};

pub enum StyleSheet {
    External(Url),
    Inline(CascadingStyleSheet),
}

impl<'a> From<StyleSheet> for Node<'a> {
    fn from(value: StyleSheet) -> Self {
        match value {
            StyleSheet::External(url) => {
                let el = Element::external_style(url);
                el.into()
            }
            StyleSheet::Inline(string) => {
                let el = Element::inline_style(Text::create(string));
                el.into()
            }
        }
    }
}
