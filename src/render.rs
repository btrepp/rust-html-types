///! Converts the node types into a string
///! This will correctly call child elements
///! Thus allowing us to format an arbitrary
///! Tree of nodes into html
///!
///! NOTE: Currently functional, but not pretty-printed
use crate::{attributes, node::*, text::Text};
use attributes::{Attribute, Value};
use std::collections::HashMap;

impl<'a> From<Node<'a>> for String {
    fn from(value: Node<'a>) -> Self {
        match value {
            Node::Text(t) => text_to_string(t),
            Node::Comment(c) => comment_to_string(c),
            Node::Element(e) => element(e),
            Node::Void(v) => void_element(v),
        }
    }
}

fn text_to_string(value: Text) -> String {
    value.into()
}

fn comment_to_string(value: Comment) -> String {
    let text: String = value.into();
    format!("<!--{}-->", text)
}

fn attributes_to_string<'a>(
    value: HashMap<attributes::Attribute<'a>, Option<attributes::Value>>,
) -> Option<String> {
    let seperator = " ";
    let joiner = "=";
    let property = |arg: (&Attribute, &Option<Value>)| -> String {
        let (key, value) = arg;
        match value {
            None => format!("{}", key),
            Some(value) => format!("{}{}{}", key, joiner, value),
        }
    };

    match value.len() {
        0 => None,
        _ => value
            .iter()
            .map(property)
            .collect::<Vec<String>>()
            .join(seperator)
            .into(),
    }
}

fn void_element(value: Element<()>) -> String {
    let tag: &str = value.name.into();
    let attributes = attributes_to_string(value.attributes);
    match attributes {
        None => format!("<{}/>", tag),
        Some(attributes) => format!("<{} {} />", tag, attributes),
    }
}

fn element(value: Element<Vec<Node>>) -> String {
    let tag: &str = value.name.into();
    let attributes = attributes_to_string(value.attributes);
    let string_child = |node: &Node| -> String { node.clone().into() };
    let child_sep = "";
    let children = value
        .children
        .iter()
        .map(string_child)
        .collect::<Vec<String>>()
        .join(child_sep);
    match attributes {
        None => format!("<{}>{}</{}>", tag, children, tag),
        Some(attr) => format!("<{} {}>{}</{}>", tag, attr, children, tag),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag::Tag;
    use attributes::{Attribute, Value};
    #[test]
    fn render_text() {
        let text = Node::text("Hello");
        let rendered: String = text.into();
        let expected = "Hello";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_comment() {
        let text = Node::comment("Hello");
        let rendered: String = text.into();
        let expected = "<!--Hello-->";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_element_void() {
        let element: Node = Element::<()>::create(Tag::BR).into();
        let expected = "<br/>";
        let render: String = element.into();
        assert_eq!(render, expected);
    }

    #[test]
    fn render_element_void_with_attributes() {
        let mut element = Element::<()>::create(Tag::BR);
        element.add_attribute(Attribute::CLASS, Value::create("test"));
        let expected = "<br class=test />";
        let node: Node = element.into();
        let render: String = node.into();
        assert_eq!(render, expected);
    }

    #[test]
    fn render_element_open() {
        let mut element = Element::<Vec<Node>>::create(Tag::A);
        let nested: Node = Text::create("Link").into();
        element.push(nested);
        let node: Node = element.into();
        let rendered: String = node.into();
        let expected = "<a>Link</a>";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_element_multi_nested() {
        let div = Element::<Vec<Node>>::create(Tag::DIV);
        let mut div2 = div.clone();
        let mut div3 = div.clone();
        div2.push(div);
        div3.push(div2);

        let node: Node = div3.into();
        let rendered: String = node.into();
        let expected = "<div><div><div></div></div></div>";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_element_two_children() {
        let div = Element::<Vec<Node>>::create(Tag::DIV);
        let div2 = div.clone();
        let mut div3 = div.clone();
        div3.push(div);
        div3.push(div2);

        let node: Node = div3.into();
        let rendered: String = node.into();
        let expected = "<div><div></div><div></div></div>";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_element_open_but_empty() {
        let element = Element::<Vec<Node>>::create(Tag::A);
        let node: Node = element.into();
        let rendered: String = node.into();
        let expected = "<a></a>";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_element_open_with_attributes() {
        let mut element = Element::<Vec<Node>>::create(Tag::A);
        element.add_attribute(Attribute::CLASS, Value::create("test"));
        let nested: Node = Text::create("Link").into();
        element.push(nested);
        let node: Node = element.into();
        let rendered: String = node.into();
        let expected = "<a class=test>Link</a>";
        assert_eq!(rendered, expected);
    }
}
