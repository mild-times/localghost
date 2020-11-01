use localghost::dom::{self, Element, ElementKind};
use localghost::prelude::*;
use localghost::keyboard::Keyboard;

#[localghost::main]
async fn main() {
    // Connect the `Keyboard`.
    let keyboard = Keyboard::new();

    // Create a table
    let table = Element::new(ElementKind::Table);
    dom::body().append_child(&table);

    // Create the headings
    let tr = Element::new(ElementKind::Tr);
    tr.append_child(Element::with_text(ElementKind::Th, "key name"));
    table.append_child(tr);

    // For every keyboard event add an entry to the table.
    let mut keydown = keyboard.keydown();
    while let Some(ev) = keydown.next().await {
        let tr = Element::new(ElementKind::Tr);
        tr.append_child(Element::with_text(ElementKind::Td, &ev.key()));
        table.append_child(tr);
    };
}
