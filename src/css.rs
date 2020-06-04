pub struct CascadingStyleSheet(String);
impl CascadingStyleSheet {
    pub fn create(text:String) -> Self {
        CascadingStyleSheet(text)
    }
}



impl From<CascadingStyleSheet> for String {
    fn from(value: CascadingStyleSheet) -> Self {
        value.0
    }

}