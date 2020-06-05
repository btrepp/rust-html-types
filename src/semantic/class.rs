pub struct Class(String);

pub struct InvalidClass();

impl Class {
    pub fn create(class:String) -> Result<Class,InvalidClass>{
        Ok(Class(class))
    }
}