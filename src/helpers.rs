///! Module with some extra constructors using public interfaces
///! Not strictly necessary to use, but does eliminate some repetitive tasks
use crate::{
    attributes,
    node::{Comment, Element, ElementType, Node},
    tag::Tag,
    text::Text, url::Url,
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

impl<'a> Element<'a,()> {


    /// An external stylesheet (link tag)
    pub fn external_style(url:Url) -> Self {
        let mut el = Element::<()>::create(Tag::LINK);
        let url: Value<'a> = url.into();
        el.set_attribute(Attribute::REL, Value::STYLESHEET);
        el.set_attribute(Attribute::HREF, url);
        el
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

    /// Helper function to create a link
    /// given the label and url
    pub fn anchor(url: Value<'a>, label: Text) -> Self {
        // Note: in future URL should be more strongly typed?
        let mut element = Element::<Vec<Node>>::create(Tag::A);
        element.set_attribute(Attribute::HREF, url);
        element.push(label);
        element.into()
    }

    /// Creates a body element. Note consumes the children vector
    pub fn body<N> (children:Vec<N>) -> Self 
        where N : Into<Node<'a>> {
        let mut el = Element::<Vec<Node>>::create(Tag::BODY);
        for child in children{
            el.push(child);
        }
        el
    }

    /// Creates the html element
    /// Helper here assumes you have a body and header.
    /// as thats's used pretty much always
    pub fn html(lang :Value<'a>, header:Element<'a,Vec<Node<'a>>>, body:Element<'a,Vec<Node<'a>>>) -> Self {
        let mut el = Element::<Vec<Node>>::create(Tag::HTML);
        el.set_attribute(Attribute::LANG,lang);
        el.push(header);
        el.push(body);
        el
    }

    /// Creates a html title, with the supplied text
    pub fn title(title:Text) -> Self {
        let mut el = Element::<Vec<Node>>::create(Tag::TITLE);
        el.push(title);
        el
    }

    /// Creates an inline script in to be used in the head section
    pub fn inline_script(text:Text) -> Self {
        let mut el = Element::<Vec<Node>>::create(Tag::SCRIPT);
        el.push(text);
        el
    }

    /// An external script tag
    pub fn external_script(url:Url) -> Self {
        let mut el = Element::<Vec<Node>>::create(Tag::SCRIPT);
        let url: Value<'a> = url.into();
        el.set_attribute(Attribute::SRC, url);
        el
    }
    

    /// Creates an inline script in to be used in the head section
    pub fn inline_style(text:Text) -> Self {
        let mut el = Element::<Vec<Node>>::create(Tag::STYLE);
        el.push(text);
        el
    }

    /// Creates the head section of a document
    pub fn head() -> Self {
        Element::<Vec<Node>>::create(Tag::HEAD)
    }

    /// Creats a main, used to hold bulk of page
    pub fn main() -> Self {
        Element::<Vec<Node>>::create(Tag::MAIN)
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

    /// Sets the supplied attribute as a 'boolean' attribute
    /// This means it will just appear in the html, and will not render
    /// with a =value eg <div editable></div>
    pub fn set_bool_attribute(&mut self, key: Attribute<'a>) {
        self.attributes.insert(key, None);
    }

    /// Sets the attribute to the supplied value
    /// Note: sugar over wrapping the value with some.
    pub fn set_attribute(&mut self, key: Attribute<'a>, value: Value<'a>) {
        self.attributes.insert(key, Some(value));
    }
}
