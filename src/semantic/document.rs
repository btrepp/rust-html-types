use super::Html;
use crate::node::Node;

pub struct Document {
    pub html: Html,
}

impl<'a> From<Document> for String {
    fn from(value: Document) -> Self {
        let node: Node<'a> = value.html.into();
        let text: String = node.into();
        format!("<!DOCTYPE html>{}", text)
    }
}
