///! A loose representation of the tree structure HTML follows
///! This can still be used to generate 'invalid' html.
///! Eg a br tag with children would be possible. However most browsers
///! are incredibly lax, as is HTML somewhat, so this is at least highly useable
///! If you know what tags you need
///!
///! This file attempts to be the full set of tools needed, so there is no usage
///! Of traits allowing further 'custom' types. It is more Data driven in it's approach
///!
///! Note: All datastructures here are fairly public, allowing them to be manipulated
///! as desired 
use std::collections::HashMap;
use derive_more::{From,Into};

/// Describes all potential shapes of a html element
/// Note that there are only three kinds, text nodes, comment nodes, and element nodes
#[derive(Clone)]
pub enum Node {
    Text(Text),
    Comment(Comment),
    Element(Element<Normal>),
    Void(Element<Void>)
}

/// The HTML text node. This is used inside tags eg <p>Text</p>
#[derive(From,Into,Clone)]
pub struct Text(String);

/// A Html comment node. <!---- Text --->
#[derive(From,Into,Clone)]
pub struct Comment(String);

pub trait ElementType: private::Sealed { }

#[derive(Clone)]
pub struct Void();
#[derive(From,Into,Clone)]
pub struct Normal(Vec<Node>);

impl ElementType for Void {}
impl ElementType for Normal {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Void {}
    impl Sealed for super::Normal {}
}

/// The html element type. This is the most common
/// Note: if children is None, then it is handled as an empty
/// element, this is different than having no children
#[derive(Clone)]
pub struct Element<T>
    where T: ElementType {
    pub name: String,
    pub attributes: HashMap<String,String>,
    pub children: T
}

impl Element<Normal> {

    /// Creates a typical element with children, from the
    /// provided tag name. This is the typical case
    pub fn create(name:String) -> Self {
        let attributes = HashMap::default();
        let children = Vec::default().into();
        Element {
            name,
            attributes,
            children
        }
    }

    pub fn push(&mut self,node: Node) {
        self.children.0.push(node);        
    }
}

impl Element<Void> {
    /// Creates an empty element. Children are None
    /// Tag would be closed when rendered
    pub fn new(name:String) -> Self {
        let attributes = HashMap::default();
        let children = Void {};
        Element {
            name, 
            children,
            attributes
        }
    }
}


mod render {
    use super::{Text,Comment,Element,Normal};
    trait RenderHtml {
        fn to_html(self) -> String;
    }
    
    impl RenderHtml for Text {
        fn to_html(self) -> String {
            self.into()            
        }
    }

    impl RenderHtml for Comment {
        fn to_html(self) -> String {
            let text :String = self.into();
            let str = format!("<!--{}-->",text);
            str
        }
    }

    impl RenderHtml for Element<Normal> {   
        fn to_html(self) -> String {
            "Hello".to_string()
        }

    }
  
    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::*;
        #[test]
        fn render_text() {
            let text :String = "Hello".into();
            let text : Text = text.into();
            let rendered = text.to_html();
            let expected = "Hello";
            assert_eq!(rendered,expected);
        }
        
        #[test]
        fn render_comment() {
            let text :String = "Hello".into();
            let text :Comment = text.into();
            let rendered = text.to_html();
            let expected = "<!--Hello-->";
            assert_eq!(rendered,expected);
        }

        #[test]
        fn render_element_open() {
            let mut element = Element::<Normal>::create("a".to_string());
            let nested : Node= Node::Text("Link".to_string().into());
            element.push(nested);
            let rendered = element.to_html();
            let expected = "<a>Link</a>";
            assert_eq!(rendered,expected);
        }


    }

}
