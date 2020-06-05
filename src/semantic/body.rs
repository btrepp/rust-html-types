use super::{class::Class, id::Id, main::Main, Script};
use crate::{
    node::{Element, Node},
    text::Text,
};

pub struct Body {
    pub content: Vec<BodyElement>,
    pub scripts: Vec<Script>,
    pub id: Option<Id>,
    pub class: Vec<Class>,
}

pub enum BodyElement {
    Text(Text),
    Main(Main),
}

impl<'a> From<BodyElement> for Node<'a> {
    fn from(value: BodyElement) -> Self {
        match value {
            BodyElement::Text(text) => text.into(),
            BodyElement::Main(main) => main.into(),
        }
    }
}

impl<'a> From<Text> for BodyElement {
    fn from(value: Text) -> Self {
        BodyElement::Text(value)
    }
}

impl<'a> From<Body> for Element<'a, Vec<Node<'a>>> {
    fn from(value: Body) -> Self {
        let empty: Vec<Node> = vec![];
        let mut body: Element<Vec<Node>> = Element::body(empty);

        for item in value.content {
            let el: Node = item.into();
            body.push(el);
        }

        for script in value.scripts {
            let el: Node = script.into();
            body.push(el);
        }
        body
    }
}
