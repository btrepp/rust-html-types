// Macro that can make a 'generic' flow content style node
// Note, only useful when we don't have more specific behaviour

use super::FlowContent;

macro_rules! elem {
    ($name:ident $tag:ident $content:ident) => {

            pub struct $name {
                pub children: Vec<$content>,
                pub id: Option<crate::semantic::id::Id>,
                pub class : Vec<crate::semantic::class::Class>
            }

            impl<'a> From<$name> for crate::node::Element<'a,Vec<crate::node::Node<'a>>> {
                fn from(value: $name) -> Self {
                    let mut el = crate::node::Element::create(crate::tag::Tag::$tag);
                    let id : Option<crate::attributes::Value> = value.id.map(Into::into);
                    let class :Option<crate::attributes::Value> = 
                    std::convert::TryInto::try_into(value.class).ok();
            
                    if let Some(id) = id {
                        el.set_attribute(crate::attributes::Attribute::ID,id);
                    }
            
                    if let Some(class) = class {
                        el.set_attribute(crate::attributes::Attribute::CLASS,class);
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
            impl<'a> From<$name> for crate::node::Node<'a> {
                fn from(value: $name) -> Self {
                    let el : crate::node::Element<'a,Vec<crate::node::Node<'a>>>  = value.into();
                    crate::node::Node::Element(el)
                }
            }

            impl std::default::Default for $name {
                fn default() -> Self {
                    $name { 
                        class: vec![], 
                        id: None, 
                        children: vec![]
                    }
                }
            }

        }
}


// Note: Bulk generic definitions
// TODO, Almost exclusively these should be reimplemented properly
elem!(Main MAIN FlowContent);
elem!(Div DIV FlowContent);

#[cfg(test)]
mod tests {
    use crate::{text::Text, node::{Element, Node}, semantic::{Class, Id}, attributes::{Value, Attribute}};

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
        let el: Element<Vec<Node<'static>>> = h1.into();
        let result = el.get_attribute_value(&Attribute::ID);
        let expected : Option<Value> = Value::create("id").unwrap().into();
        assert_eq!(&expected,result);
       
    }

    #[test]
    fn preserves_class() {
        let h1 = fixture();
        let el: Element<Vec<Node<'static>>> = h1.into();
        let result = el.get_attribute_value(&Attribute::CLASS);
        let expected : Option<Value> = Value::create("class").unwrap().into();
        assert_eq!(&expected,result);
    }

    #[test]
    fn preserves_content() {
        let h1 = fixture();
        let el: Element<Vec<Node<'static>>> = h1.into();
        let result = el.children[0].clone();
        let expected : Node = Text::create("text").into();
        assert_eq!(expected,result);
    }
}