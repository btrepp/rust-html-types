///! Module with some extra constructors using public interfaces
///! Not strictly necessary to use, but does eliminate some repetitive tasks

use crate::{tag::{TagParseError, Tag}, node::{Node, Comment, Text, Element, ElementType}};
use std::collections::HashMap;

impl<'a> Node<'a> {

    pub fn comment(text:&str) -> Self {
        let comment :Comment  = text.to_string().into();
        comment.into()
    }

    pub fn text(text:&str) -> Self {
        let text: Text = text.to_string().into();
        text.into()
    }
    
}


impl<'a> Element<'a,Vec<Node<'a>>> {
    pub fn push<N>(&mut self,node: N) -> ()
        where N: Into<Node<'a>>  {
        let node = node.into();
        self.children.push(node);        
    }
}


impl <'a,T> Element<'a,T> 
    where T: ElementType {

    /// Creates a typical element with children, from the
    /// provided tag name. This is the typical case
    pub fn create(name:Tag<'a>) -> Self {
        let attributes = HashMap::default();
        let children = T::default();
        let element = 
            Element {
                name,
                attributes,
                children
            };
        element
    }

    pub fn try_create<S>(name:S) -> Result<Self,TagParseError> 
        where S : Into<&'a str> {
        let text = name.into();
        let tag = Tag::try_create(text)?;
        let element = Self::create(tag);
        Ok(element)
    }

}
