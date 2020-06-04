

use super::{Body, Html, Head, Script, StyleSheet, BodyElement, Document};
use crate::{ node::{Element, Node}, text::Text};

impl<'a> From<Script> for Node<'a> {
    fn from(value: Script) -> Self {
        match value {
            Script::External(url) => {
                let el = Element::external_script(url);
                el.into()
            },
            Script::Inline(string) => {
                let el = Element::inline_script(Text::create(string));
                el.into()
            }
        }
    }
}


impl<'a> From<StyleSheet> for Node<'a> {
    fn from(value: StyleSheet) -> Self {
        match value {
            StyleSheet::External(url) => {
                let el = Element::external_style(url);
                el.into()
            },
            StyleSheet::Inline(string) => {
                let el = Element::inline_style(Text::create(string));
                el.into()
            }
        }
    }
}

impl<'a> From<Body> for Element<'a,Vec<Node<'a>>> {
    fn from(value: Body) -> Self {
        let empty : Vec<Node> = vec![];
        let mut body: Element<Vec<Node>> = Element::body(empty);
        
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

impl<'a> From<Head> for Element<'a,Vec<Node<'a>>> {
    fn from(value: Head) -> Self {
        let mut head = Element::head();
        match value.title {
            Some(title) => {
                let title = Element::title(title);
                head.push(title)
            },
            None => ()
        };

        for style in value.styles {
            let el: Node = style.into();
            head.push(el);
        }

        for script in value.scripts {
            let el: Node = script.into();
            head.push(el);
        }

        head
    }
}

impl<'a> From<BodyElement> for Node<'a> {
    fn from(value: BodyElement) -> Self {
        match value {
            BodyElement::Text(text) => text.into()
        }
    }
}

impl<'a> From<Text> for BodyElement {
    fn from(value: Text) -> Self {
        BodyElement::Text(value)
    }
}
impl<'a> From<Html> for Element<'a,Vec<Node<'a>>>{
    fn from(value: Html) -> Self {
        let header: Element<Vec<Node<'a>>> = value.head.into();
        let body: Self = value.body.into();
        Element::html(value.lang,header, body)
    }

}

impl<'a> From<Html> for Node<'a> {
    fn from(html: Html) -> Self {
        let el : Element<Vec<Node<'a>>> = html.into();
        el.into()
    }
}

impl<'a> From<Document> for String {
    fn from(value: Document) -> Self {
        let node : Node<'a> = value.html.into();
        let text : String = node.into();
        format!("<!DOCTYPE html>{}",text)
    }

}