use html_types::semantic::{Body, Class, Document, Head, Html, Id, Script};
use html_types::{attributes::Value, text::Text, url::Url};

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
    let url = Url::absolute_unchecked("http://google.com".into());
    let script = Script::External(url);
    let scripts = vec![script];
    let head = Head {
        title,
        styles,
        scripts,
    };
    let content = vec![Text::create("Hello").into()];
    let body = Body {
        content,
        scripts: vec![],
        id: Some(Id::create("my-id").unwrap()),
        class: vec![
            Class::create("test").unwrap(),
            Class::create("body").unwrap(),
        ],
    };
    let html = Html {
        head,
        body,
        lang: Value::EN,
    };
    let doc = Document { html };
    let string: String = doc.into();

    println!("{}", string);
}
