use super::{
    class::{Class, EmptyVector},
    id::Id,
    Script, FlowContent,
};
use crate::{
    attributes::{Attribute, Value},
    node::{Element, Node}
};
use std::convert::TryInto;

pub struct Body {
    pub content: Vec<FlowContent>,
    pub scripts: Vec<Script>,
    pub id: Option<Id>,
    pub class: Vec<Class>,
}


impl<'a> From<Body> for Element<'a, Vec<Node<'a>>> {
    fn from(value: Body) -> Self {
        let empty: Vec<Node> = vec![];
        let mut body: Element<Vec<Node>> = Element::body(empty);

        match value.id {
            Some(id) => body.set_attribute(Attribute::ID, id.into()),
            None => (),
        };

        let class_value: Result<Value<'a>, EmptyVector> = value.class.try_into();
        match class_value {
            Ok(value) => body.set_attribute(Attribute::CLASS, value),
            Err(_) => {}
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::{Attribute, Value};

    #[test]
    fn preserves_id() {
        let id = Id::create("abc").unwrap();
        let id = Some(id);
        let body = Body {
            content: vec![],
            scripts: vec![],
            id,
            class: vec![],
        };
        let el: Element<Vec<Node>> = body.into();
        let result = el.has_attribute(&Attribute::ID);
        assert!(result);
    }

    #[test]
    fn preserves_class() {
        let class = Class::create("abc").unwrap();
        let body = Body {
            content: vec![],
            scripts: vec![],
            id: None,
            class: vec![class],
        };
        let el: Element<Vec<Node>> = body.into();
        let result = el.get_attribute_value(&Attribute::CLASS);
        let expected = {
            let value = Value::owned("abc").unwrap();
            &Some(value)
        };
        assert_eq!(result, expected);
    }
}
