use super::{Body, Head};
use crate::{
    attributes::Value,
    node::{Element, Node},
};

pub struct Html {
    pub lang: Value<'static>,
    pub head: Head,
    pub body: Body,
}

impl<'a> From<Html> for Element<'a, Vec<Node<'a>>> {
    fn from(value: Html) -> Self {
        let header: Element<Vec<Node<'a>>> = value.head.into();
        let body: Self = value.body.into();
        Element::html(value.lang, header, body)
    }
}

impl<'a> From<Html> for Node<'a> {
    fn from(html: Html) -> Self {
        let el: Element<Vec<Node<'a>>> = html.into();
        el.into()
    }
}
