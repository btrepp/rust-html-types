use derive_more::Display;

#[derive(Debug)]
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
        match str.chars().all(char::is_alphabetic) {
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