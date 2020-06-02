use derive_more::Display;

#[derive(Debug,PartialEq,Eq)]
pub struct InvalidValueError {}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Value<'a>(&'a str);

macro_rules! value {
    ($name:ident $tag:expr) => {
        pub const $name: Value<'static> = Value($tag);
    };
}


impl<'a> Value<'a> {
    pub fn create(str: &'a str) -> Result<Value<'a>,InvalidValueError>
    {
        let allowed = |c:char| -> bool {
            char::is_alphabetic(c)
            || c == ':'
            || c == '/'
            || c == '.'
        };
        match str.chars().all(allowed) {
            true => Ok(Value(str)),
            false => Err (InvalidValueError{})
        }
    }

    // Common Html Attribute values
    // Used to have easier 'static' access to these 'constants'

    value!(TEXT_CSS "text/css");
    value!(STYLESHEET "stylesheet");
    value!(UTF_8 "UTF-8");    

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn url_is_valid() {
        let url = "http://google.com";
        let node = Value::create(url);
        let expected = Result::Ok(Value(url));
        assert_eq!(node, expected);
    }
}