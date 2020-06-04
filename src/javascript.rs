pub struct Javascript(String);

impl Javascript {
    pub fn create(text:String) -> Self {
        Javascript(text)
    }
}

impl From<Javascript> for String {
    fn from(value: Javascript) -> Self {
        value.0
    }

}