use derive_more::Into;
/// A html tag definition
#[derive(Into,Clone)]
pub struct Tag<'a>(&'a str);

/// Returned when the tag didn't contain valid characters
pub struct TagParseError {}

impl<'a> Tag<'a> {

    /// Try to create the tag from a string
    /// Note this will currently fail if the tag is not alphabetic
    ///
    /// Prefer to use the contant forms if possible. However HTML can have custom
    /// tags, so we do need this possibility
    pub fn try_create<S>(text:S) -> Result<Self,TagParseError> 
        where S: Into<&'a str> {
        let string = text.into();
        match string.chars().all(char::is_alphabetic) {
            false => Err(TagParseError {}),
            true => {
                let tag = Tag(string);
                Ok(tag)
            }
        }
    }

    pub const DIV : Self = Tag("div");
    pub const BR : Self = Tag("br");
    pub const A: Self = Tag("a");
}