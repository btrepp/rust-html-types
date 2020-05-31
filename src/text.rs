use derive_more::Into;
/// The HTML text node. This is used inside tags eg <p>Text</p>
#[derive(Into, Clone,PartialEq,Eq,Debug)]
pub struct Text(String);

pub struct InvalidTextError();
impl Text {

    /// Creates a text node. Note: will escape html markup eg <,>,&
    pub fn create<S>(text:S) -> Text 
        where S: Into<String> {
            // I suspect there might be a more complete ruleset here
            let text = text.into();
            let gt = ">";
            let lt = "<";
            let amp = "&";
            let gt_escaped = "&gt;";
            let lt_escaped = "&lt;";
            let amp_escaped = "&amp;";
            let text = 
                text.replace(amp,amp_escaped)
                    .replace(gt,gt_escaped)
                    .replace(lt,lt_escaped);
            Text(text)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unescaped() {
        let input = Text::create("Hello");
        let expected = Text("Hello".to_string());
        assert_eq!(input,expected);
    }

    #[test]
    fn escaped_greater_than() {
        let input = Text::create(">attempt");
        let expected = Text("&gt;attempt".to_string());
        assert_eq!(input,expected);
    }

    #[test]
    fn escaped_less_than() {
        let input = Text::create("<attempt");
        let expected = Text("&lt;attempt".to_string());
        assert_eq!(input,expected);
    }

    #[test]
    fn escaped_ampersand() {
        let input = Text::create("&attempt");
        let expected = Text("&amp;attempt".to_string());
        assert_eq!(input,expected);
    }
}