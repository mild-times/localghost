use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, query_selector};

use std::cell::RefCell;
use std::rc::Rc;

#[localghost::main]
async fn main() {
    // TODO: this should be part of the macro.
    localghost::log::Logger::new().start().unwrap_throw();

    let count = Rc::new(RefCell::new(0isize));
    let body = localghost::document().body();

    let button = Element::with_text(ElementKind::Button, "+");
    let count_handle = count.clone();
    button.on("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle += 1;
        let counter = query_selector("#counter").unwrap_throw();
        counter.set_text_content(Some(&format!("{}", *handle)))
    });
    body.append_child(button);


    let p = Element::with_text(ElementKind::P, &format!("{}", count.borrow()));
    p.set_attribute("id", "counter");
    body.append_child(p);

    let button = Element::with_text(ElementKind::Button, "-");
    let count_handle = count.clone();
    button.on("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle -= 1;
        let counter = query_selector("#counter").unwrap_throw();
        counter.set_text_content(Some(&format!("{}", *handle)));
    });
    body.append_child(button);
}
