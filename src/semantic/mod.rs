mod to_element;

///! Experimental
///! Attempts to provide defined structure
///! So that pages are more semantically checked
///!
///! This probably doesn't cover all 'correct' cases
///! of html

use crate::text::Text;
use crate::url::Url;

pub struct Javascript(String);
pub struct CascadingStyleSheet(String);

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
    pub head: Head,
    pub body: Body
}

pub struct Body {
    pub content: Text,
    pub scripts: Vec<Script>
}

impl Javascript {
    pub fn create(text:String) -> Self {
        Javascript(text)
    }
}

impl CascadingStyleSheet {
    pub fn create(text:String) -> Self {
        CascadingStyleSheet(text)
    }
}

impl From<Javascript> for String {
    fn from(value: Javascript) -> Self {
        value.0
    }

}

impl From<CascadingStyleSheet> for String {
    fn from(value: CascadingStyleSheet) -> Self {
        value.0
    }

}