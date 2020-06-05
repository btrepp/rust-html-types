pub struct Id(String);

pub struct InvalidId();

impl Id {
    pub fn create(id:String) -> Result<Id,InvalidId>{
        Ok(Id(id))
    }
}