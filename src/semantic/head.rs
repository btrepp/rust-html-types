use super::{Script, StyleSheet};
use crate::{
    node::{Element, Node},
    text::Text,
};

pub struct Head {
    pub title: Option<Text>,
    pub styles: Vec<StyleSheet>,
    pub scripts: Vec<Script>,
}

impl<'a> From<Head> for Element<'a, Vec<Node<'a>>> {
    fn from(value: Head) -> Self {
        let mut head = Element::head();
        match value.title {
            Some(title) => {
                let title = Element::title(title);
                head.push(title)
            }
            None => (),
        };

        for style in value.styles {
            let el: Node = style.into();
            head.push(el);
        }

        for script in value.scripts {
            let el: Node = script.into();
            head.push(el);
        }

        head
    }
}
