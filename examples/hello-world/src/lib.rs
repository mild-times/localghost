use localghost::prelude::*;
use localghost::dom::{self, Element, ElementKind, Text};

#[localghost::main]
async fn main() {
    let p = Element::with_text(ElementKind::P, "Hello world");
    dom::body().append(p);
}
