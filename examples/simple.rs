// examples/hello.rs

use html_types::attributes::Value;
use html_types::node::Element;
use html_types::node::Node;
use html_types::tag::Tag;
use html_types::text::Text;

fn main() {
    // Create a link
    let link = {
        let label = Text::create("Click Me");
        let url = Value::create("http://google.com").unwrap();
        // Anchor is a helper for the typical case
        Element::anchor(url, label)
    };
    // Create the body. Sugar function takes a list of child nodes
    let body = Element::body(vec![link]);

    // Create a header manually. There isn't a sugar function here
    let header = {
        let mut el = Element::<Vec<Node>>::create(Tag::HEADER);
        let text = Text::create("Hello world");
        let title = Element::title(text);
        el.push(title);
        el
    };
    let html = Element::html(Value::EN, header, body);

    // Convert an element into a node
    let node: Node = html.into();

    // Nodes can be turned into HTML formatted strings
    let string: String = node.into();

    println!("{}", string);
}
