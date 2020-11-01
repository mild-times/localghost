use localghost::prelude::*;
use localghost::dom::{self, Element, ElementKind, query_selector};

use std::cell::RefCell;
use std::rc::Rc;

#[localghost::main]
async fn main() {
    // TODO: this should be part of the macro.
    localghost::log::Logger::new().start().unwrap_throw();

    let counter = Rc::new(RefCell::new(0isize));
    let body = dom::body();

    let button = Element::with_text(ElementKind::Button, "+");
    let count_handle = counter.clone();
    button.on_with("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle += 1;
        let el = query_selector("#counter").unwrap_throw();
        el.set_text(&format!("{}", *handle))
    });
    body.append(button);

    let p = Element::with_text(ElementKind::P, &format!("{}", counter.borrow()));
    p.set_attr("id", "counter");
    body.append(p);

    let button = Element::with_text(ElementKind::Button, "-");
    let count_handle = counter.clone();
    button.on_with("click", move |_| {
        let mut handle = count_handle.borrow_mut();
        *handle -= 1;
        let el = query_selector("#counter").unwrap_throw();
        el.set_text(&format!("{}", *handle));
    });
    body.append(button);
}
