use localghost::prelude::*;
use localghost::events::EventListener;
use localghost::dom::{Element, ElementKind, Text};

use std::cell::RefCell;

#[localghost::main]
async fn main() {
    let count = RefCell::new(0usize);
    let mut body = localghost::document().body();

    let mut button = Element::new(ElementKind::Button);
    let count2 = count.clone();
    EventListener::listen(&button, "click", move |_| {
        *count2.borrow_mut() += 1;
    }).forget();
    button.append_child(Text::new("+"));
    body.append_child(button);

    body.append_child(Text::new(&format!("{}", count.borrow())));

    let mut button = Element::new(ElementKind::Button);
    button.append_child(Text::new("-"));
    let count2 = count.clone();
    EventListener::listen(&button, "click", move |_| {
        *count2.borrow_mut() += 1;
    }).forget();
    body.append_child(button);
}
