use localghost::dom::{query_selector, self, Element, ElementKind};
use localghost::prelude::*;
use localghost::raw::web_sys;

#[localghost::main]
async fn main() {
    // Access the document's `<body>`.
    let body = dom::body();

    // Create an `<input>` field
    let input = Element::new(ElementKind::Input);
    input.set_attribute("type", "text");
    input.set_attribute("placeholder", "What's your name?");
    input.on("input", |ev| {
        if let Some(target) = ev.target::<web_sys::HtmlInputElement>() {
            let el = query_selector("#text").unwrap_throw();
            el.set_text_content(Some(&target.value()));
        }
    });
    body.append_child(input);

    // Create a `<p>` node to display the form's output.
    let text = Element::new(ElementKind::P);
    text.set_attribute("id", "text");
    body.append_child(text);
}
