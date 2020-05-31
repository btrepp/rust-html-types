///! Module with some extra constructors using public interfaces
///! Not strictly necessary to use, but does eliminate some repetitive tasks
use crate::{
    attributes,
    node::{Comment, Element, ElementType, Node},
    tag::Tag,
    text::Text,
};
use attributes::{Attribute, Value};
use std::collections::HashMap;

impl<'a> Node<'a> {
    /// Creates a comment as a Node from the supplied string
    pub fn comment(text: &str) -> Self {
        let comment: Comment = text.to_string().into();
        comment.into()
    }

    /// Creates a text element as a node
    pub fn text(text: &str) -> Self {
        let text: Text = Text::create(text);
        text.into()
    }
}

impl<'a> Element<'a, Vec<Node<'a>>> {
    /// Pushes a child not into the element
    /// Note: you can also do this yourself, but this is nicer
    /// as it will coerce anything that can be a node into a node
    pub fn push<N>(&mut self, node: N) -> ()
    where
        N: Into<Node<'a>>,
    {
        let node = node.into();
        self.children.push(node);
    }
}

impl<'a, T> Element<'a, T>
where
    T: ElementType,
{
    /// Creates a typical element with children, from the
    /// provided tag name. This is the typical case
    pub fn create(name: Tag<'a>) -> Self {
        let attributes = HashMap::default();
        let children = T::default();
        let element = Element {
            name,
            attributes,
            children,
        };
        element
    }

    pub fn add_bool_attribute(&mut self, key: Attribute<'a>) {
        self.attributes.insert(key, None);
    }

    pub fn add_attribute(&mut self, key: Attribute<'a>, value: Value) {
        self.attributes.insert(key, Some(value));
    }
}
