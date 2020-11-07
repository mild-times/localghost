use localghost::dom::{self, Element};
use localghost::prelude::*;
use localghost::keyboard::Keyboard;

use futures::stream::StreamExt;

#[localghost::main]
async fn main() {
    let keyboard = Keyboard::new();
    let body = dom::body();

    let desc = Element::with_text("p", "Press a key, get a key name");
    body.append(desc);

    let heading = Element::new("h1");
    heading.set_attr("id", "target");
    body.append(heading);

    // For every keyboard event modify the heading.
    let mut keydown = keyboard.key_down();
    while let Some(ev) = keydown.next().await {
        let el = dom::query_selector("#target").unwrap_throw();
        el.set_text(&ev.key().to_string());
    };
}
