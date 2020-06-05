use super::{class::Class, id::Id, main::Main, Script};
use crate::text::Text;

pub struct Body {
    pub content: Vec<BodyElement>,
    pub scripts: Vec<Script>,
    pub id: Option<Id>,
    pub class: Vec<Class>,
}

pub enum BodyElement {
    Text(Text),
    Main(Main),
}
