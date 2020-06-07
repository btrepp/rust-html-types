#![macro_use]

// Macro that can make a 'generic' flow content style node
// Note, only useful when we don't have more specific behaviour

macro_rules! elem {
    ($name:ident $tag:ident $content:ident) => {
        use crate::semantic::id::Id;
        use crate::tag::Tag;
        use crate::semantic::class::Class;
        use crate::attributes::Value;
        use crate::attributes::Attribute;
        use crate::node::Node;
        use std::convert::TryInto;

        type Element<'a> = crate::node::Element<'a,Vec<Node<'a>>>;
        
        pub struct $name {
            pub children: Vec<$content>,
            pub id: Option<Id>,
            pub class : Vec<Class>
        }

        impl<'a> From<$name> for Element<'a> {
            fn from(value: $name) -> Self {
                let mut el = Element::create(Tag::$tag);
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

        // Note, while this could be more generic
        // its a bit difficult to implement blanket types
        // without specialization
        impl<'a> From<$name> for Node<'a> {
            fn from(value: $name) -> Self {
                let el : Element<'a> = value.into();
                Node::Element(el)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{text::Text};

    pub struct TestContent();
    impl<'a> From<TestContent> for Node<'a> {
        fn from(_: TestContent) -> Self {
            Text::create("text").into()
        }
    }

    elem!(Main MAIN TestContent);

    fn fixture() -> Main {
        let id = Id::create("id").unwrap().into();
        let class = vec![Class::create("class").unwrap()];
        let children = vec![TestContent()];
        Main {
            id, class, children
        }
    }
    #[test]
    fn preserves_id() {
        let h1 = fixture();
        let el: Element = h1.into();
        let result = el.get_attribute_value(&Attribute::ID);
        let expected : Option<Value> = Value::create("id").unwrap().into();
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
        let expected : Node = Text::create("text").into();
        assert_eq!(expected,result);
    }
}