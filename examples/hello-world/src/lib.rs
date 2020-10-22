use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, Text};

#[localghost::main]
async fn main() {
    let mut p = Element::new(ElementKind::P);
    p.append_child(Text::new("Hello world"));

    let mut body = localghost::document().body();
    body.append_child(p);
}
