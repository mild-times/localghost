use localghost::dom::{self, Element, ElementKind};
use localghost::prelude::*;
use localghost::keyboard::Keyboard;

use futures::stream::StreamExt;

#[localghost::main]
async fn main() {
    let keyboard = Keyboard::new();
    let body = dom::body();

    let desc = Element::with_text(ElementKind::P, "Press a key, get a key name");
    body.append_child(desc);

    let heading = Element::new(ElementKind::H1);
    heading.set_attribute("id", "target");
    body.append_child(heading);

    // For every keyboard event modify the heading.
    let mut keydown = keyboard.key_down();
    while let Some(ev) = keydown.next().await {
        let el = dom::query_selector("#target").unwrap_throw();
        el.set_text_content(Some(ev.key().to_string().as_str()));
    };
}
