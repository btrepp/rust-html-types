
use crate::{node::{Node},  tag::Tag, attributes::{Attribute, Value}};
use super::{Class, Id, PhrasingContent};
use std::convert::TryInto;

type Element<'a> = crate::node::Element<'a,Vec<Node<'a>>>;

pub struct H1 {
    pub children: Vec<PhrasingContent>,
    pub id: Option<Id>,
    pub class: Vec<Class>
}

impl<'a> From<H1> for Element<'a> {
    fn from(value: H1) -> Self {
        let mut el = Element::create(Tag::H1);
        let id : Option<Value> = value.id.map(Into::into);
        let class :Option<Value> = value.class.try_into().ok();

        if let Some(id) = id {
            el.set_attribute(Attribute::ID,id);
        }

        if let Some(class) = class {
            el.set_attribute(Attribute::CLASS,class);
        }

        for child in value.children {
            el.push(child);
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{attributes::{Attribute, Value}, text::Text};

    fn fixture() -> H1 {
        let id: Option<Id> = Id::create("abc").unwrap().into();
        let class = vec![Class::create("class").unwrap()];
        let children = vec![Text::create("Title").into()];
        let h1 = H1 { id, class , children};
        h1
    }

    #[test]
    fn preserves_id() {
        let h1 = fixture();
        let el: Element = h1.into();
        let result = el.get_attribute_value(&Attribute::ID);
        let expected : Option<Value> = Value::create("abc").unwrap().into();
        assert_eq!(&expected,result);
       
    }

    #[test]
    fn preserves_class() {
        let h1 = fixture();
        let el: Element = h1.into();
        let result = el.get_attribute_value(&Attribute::CLASS);
        let expected : Option<Value> = Value::create("class").unwrap().into();
        assert_eq!(&expected,result);
    }

    #[test]
    fn preserves_content() {
        let h1 = fixture();
        let el: Element = h1.into();
        let result = el.children[0].clone();
        let expected : Node = Text::create("Title").into();
        assert_eq!(expected,result);
    }
}
