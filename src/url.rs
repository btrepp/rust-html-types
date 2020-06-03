


use crate::attributes::Value;

//TODO actually implement
pub struct AbsoluteUrl(String);

//TODO actually implement
pub struct Relative(String);

pub enum Url {
    Absolute(AbsoluteUrl),
    Relative(Relative)
}


impl<'a> From<Url> for Value<'a> {
    fn from(_: Url) -> Self {
        todo!()
    }

}