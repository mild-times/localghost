use localghost::prelude::*;
use localghost::dom::{body, Element};

#[localghost::main]
async fn main() {
    let el = Element::with_text("p", "Hello world");
    body().append(el);
}
