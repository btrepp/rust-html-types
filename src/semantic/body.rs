use super::{id::Id, Script, class::Class, main::Main};
use crate::text::Text;

pub struct Body {
    pub content: Vec<BodyElement>,
    pub scripts: Vec<Script>,
    pub id: Option<Id>,
    pub class: Vec<Class>
}

pub enum BodyElement {
    Text(Text),
    Main(Main)
}