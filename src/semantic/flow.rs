use crate:: text::Text;
use crate::node::Node;
use super::Main;
use super::Div;
/// Elements belonging to the flow content category typically contain text or embedded content. 
///
/// https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Content_categories#Flow_content
pub enum FlowContent {
    Text(Text),
    Main(Main),
    Div(Div)
}

impl From<Text> for FlowContent {
    fn from(v: Text) -> Self {
        FlowContent::Text(v)
    }
}

impl From<Main> for FlowContent {
    fn from(m:Main) -> Self {
        FlowContent::Main(m)
    }
}

impl<'a> From<FlowContent> for Node<'a> {
    fn from(value: FlowContent) -> Self {
        match value {
            FlowContent::Text(text) => text.into(),
            FlowContent::Main(main) => main.into(),
            FlowContent::Div(div) => div.into()
        }
    }
}


