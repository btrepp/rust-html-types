

use crate::{node::Node, text::Text};

/// Phrasing content defines the text and the mark-up it contains. Runs of phrasing content make up paragraphs.
/// https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Content_categories#Phrasing_content
pub enum PhrasingContent {
    Text(Text),
}


impl From<Text> for PhrasingContent {
    fn from(v: Text) -> Self {
        PhrasingContent::Text(v)
    }
}

impl<'a> From<PhrasingContent> for Node<'a> {
    fn from(value: PhrasingContent) -> Self {
        match value {
            PhrasingContent::Text(text) => text.into()
        }
    }
}