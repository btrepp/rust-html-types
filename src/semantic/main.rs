use crate::node::{Element, Node};

pub struct Main {}

impl<'a> From<Main> for Node<'a> {
    fn from(_value: Main) -> Self {
        let el = Element::main();
        el.into()
    }
}
