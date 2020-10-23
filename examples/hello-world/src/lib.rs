use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, Text};

#[localghost::main]
async fn main() {
    let p = Element::with_text(ElementKind::P, "Hello world");
    let body = localghost::document().body();
    body.append_child(p);
}
