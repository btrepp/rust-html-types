use crate::attributes::Value;

//TODO actually implement
pub struct AbsoluteUrl(String);

//TODO actually implement
pub struct Relative(String);

pub enum Url {
    Absolute(AbsoluteUrl),
    Relative(Relative),
}

impl Url {
    pub fn absolute_unchecked(text: String) -> Self {
        let abs = AbsoluteUrl(text);
        Url::Absolute(abs)
    }
}

impl<'a> From<Url> for Value<'a> {
    fn from(value: Url) -> Self {
        match value {
            Url::Absolute(s) => Value::owned(s.0).unwrap(),
            Url::Relative(s) => Value::owned(s.0).unwrap(),
        }
    }
}
