use html_types::semantic::{Main,Body,FlowContent};
use html_types::node::{Element,Node};

fn main() {


    let main : FlowContent = {
        let main :Main = Default::default();
        main.into()
    };

    let body = Body {
        content: vec![main],
        scripts: vec![],
        id: None,
        class: vec![]
    };

    let string: Element<Vec<Node>> = body.into();
    let string : String = Node::Element(string).into();
    println!("{}", string);
}
