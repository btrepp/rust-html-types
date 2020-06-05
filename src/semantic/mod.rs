mod body;
mod class;
mod id;
mod main;
mod to_element;

pub use body::*;

///! Experimental
///! Attempts to provide defined structure
///! So that pages are more semantically checked
///!
///! This probably doesn't cover all 'correct' cases
///! of html
use crate::text::Text;
use crate::{attributes::Value, css::CascadingStyleSheet, javascript::Javascript, url::Url};

pub enum StyleSheet {
    External(Url),
    Inline(CascadingStyleSheet),
}

pub enum Script {
    External(Url),
    Inline(Javascript),
}

pub struct Head {
    pub title: Option<Text>,
    pub styles: Vec<StyleSheet>,
    pub scripts: Vec<Script>,
}

pub struct Html {
    pub lang: Value<'static>,
    pub head: Head,
    pub body: Body,
}

pub struct Document {
    pub html: Html,
}
