use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, query_selector};

use std::cell::RefCell;
use std::rc::Rc;

#[localghost::main]
async fn main() {
    // TODO: this should be part of the macro.
    localghost::log::Logger::new().start().unwrap_throw();

    let counter = Rc::new(RefCell::new(0isize));
    let body = localghost::document().body();

    let button = Element::with_text(ElementKind::Button, "+");
    let count_handle = counter.clone();
    button.on("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle += 1;
        let el = query_selector("#counter").unwrap_throw();
        el.set_text_content(Some(&format!("{}", *handle)))
    });
    body.append_child(button);

    let p = Element::with_text(ElementKind::P, &format!("{}", counter.borrow()));
    p.set_attribute("id", "counter");
    body.append_child(p);

    let button = Element::with_text(ElementKind::Button, "-");
    let count_handle = counter.clone();
    button.on("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle -= 1;
        let el = query_selector("#counter").unwrap_throw();
        el.set_text_content(Some(&format!("{}", *handle)));
    });
    body.append_child(button);
}
