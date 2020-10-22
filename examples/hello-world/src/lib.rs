use localghost::prelude::*;
use localghost::dom;

#[localghost::main]
async fn main() {
    let mut p = dom::Element::new(dom::NodeKind::P);
    p.append_child(dom::Text::new("Hello world"));

    let body = localghost::document().body();
    body.append_child(p);
}
