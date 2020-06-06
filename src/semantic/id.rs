use crate::attributes::Value;

pub struct Id(String);

#[derive(Debug)]
pub struct InvalidId();

impl Id {
    pub fn create<S>(id: S) -> Result<Id, InvalidId>
    where
        S: Into<String>,
    {
        let id = id.into();
        let is_valid_value = Value::is_valid(&id);
        match is_valid_value {
            true => Ok(Id(id)),
            false => Err(InvalidId()),
        }
    }
}

impl<'a> From<Id> for Value<'a> {
    fn from(value: Id) -> Self {
        let text = value.0;
        //Note: Can only be constructed if it is a valid value
        //ID constructor checks this. So this case is always successful
        Value::owned(text).unwrap()
    }
}
