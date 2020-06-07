use derive_more::Display;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidValueError {}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Value<'a>(Cow<'a, str>);

macro_rules! value {
    ($name:ident $tag:expr) => {
        pub const $name: Value<'static> = Value(Cow::Borrowed($tag));
    };
}

impl<'a> Value<'a> {
    /// Function that determines whether a string slice would be
    /// considered a valid value. Can be helpful elsewhere when specifing
    /// more restrictive types
    pub fn is_valid(str: &str) -> bool {
        let allowed = |c: char| -> bool {
            char::is_alphabetic(c)
                || c == ':'
                || c == '/'
                || c == '.'
                || char::is_whitespace(c)
                || c == '-'
        };
        str.chars().all(allowed)
    }

    pub fn create(str: &'a str) -> Result<Value<'a>, InvalidValueError> {
        match Self::is_valid(str) {
            true => Ok(Value(Cow::Borrowed(str))),
            false => Err(InvalidValueError {}),
        }
    }

    pub fn owned<S>(str: S) -> Result<Value<'a>, InvalidValueError>
    where
        S: Into<String>,
    {
        let str = str.into();
        let valid = Self::is_valid(&str);
        match valid {
            true => Ok(Value(Cow::Owned(str))),
            false => Err(InvalidValueError {}),
        }
    }

    // Common Html Attribute values
    // Used to have easier 'static' access to these 'constants'

    value!(TEXT_CSS "text/css");
    value!(STYLESHEET "stylesheet");
    value!(UTF_8 "UTF-8");
    value!(EN "en");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_is_valid() {
        let url = "http://google.com";
        let node = Value::create(url);
        let expected = Result::Ok(Value(Cow::Borrowed(url)));
        assert_eq!(node, expected);
    }

    #[test]
    fn namespacing_is_valid() {
        let id = "test-id";
        let node = Value::create(id);
        let expected = Result::Ok(Value(Cow::Borrowed(id)));
        assert_eq!(node, expected);
    }

    
}
