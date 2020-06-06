use crate::attributes::Value;
use std::convert::TryFrom;

pub struct Class(String);

#[derive(Debug)]
pub struct InvalidClass(String);

impl Class {
    pub fn create<S>(class: S) -> Result<Class, InvalidClass>
    where
        S: Into<String>,
    {
        let string = class.into();
        let is_value = Value::is_valid(&string);
        if is_value {
            Ok(Class(string))
        } else {
            Err(InvalidClass(string))
        }
    }
}

pub struct EmptyVector;
impl<'a> TryFrom<Vec<Class>> for Value<'a> {
    type Error = EmptyVector;
    fn try_from(value: Vec<Class>) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Err(EmptyVector),
            _ => {
                let whitespace = " ";
                let to_str = |x: Class| -> String { x.0 };
                let strings = value
                    .into_iter()
                    .map(to_str)
                    .collect::<Vec<String>>()
                    .join(whitespace);
                // If all classes are valid values, and the seperator is allowed
                // then joining them should never panics
                let value = Value::owned(strings).unwrap();
                Ok(value)
            }
        }
    }
}
