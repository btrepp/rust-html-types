use html_types::semantic::{Head,Html,Body };
use html_types::node::Node;
use html_types::text::Text;

// This shows the semantic module of
// The semantic module is trying to provide strongly typed models of the html data structure
// The idea is that you are required to have values in the correct types. Such that the
// title element exists as a text node, inside head
// 
// This means html syntax errors == compiler errors
// It also means you can much more easily verify your template functions worked as expected.
// This sample builds a very minimal document. Using the semantic module
fn main() {
    let title = Some(Text::create("Title"));
    let styles = vec![];
    let scripts = vec![];
    let head = Head { title, styles, scripts };
    let content = Text::create("Hello");
    let body = Body { content, scripts : vec![]};
    let html = Html { head , body };

    let node :Node = html.into();

    let string : String = node.into();

    println!("{}", string);
}
