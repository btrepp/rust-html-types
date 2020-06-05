mod to_element;
mod body;
mod id;
mod class;
mod main;

pub use body::*;

///! Experimental
///! Attempts to provide defined structure
///! So that pages are more semantically checked
///!
///! This probably doesn't cover all 'correct' cases
///! of html

use crate::text::Text;
use crate::{css::CascadingStyleSheet, url::Url, javascript::Javascript, attributes::Value};


pub enum StyleSheet {
    External(Url),
    Inline(CascadingStyleSheet)
}

pub enum Script{
    External(Url),
    Inline(Javascript)
}

pub struct Head {
    pub title: Option<Text>,
    pub styles: Vec<StyleSheet>,
    pub scripts: Vec<Script>
}

pub struct Html {
    pub lang: Value<'static>,
    pub head: Head,
    pub body: Body
}

pub struct Document {
    pub html: Html
}