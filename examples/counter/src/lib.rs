use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, query_selector};

use std::cell::RefCell;

#[localghost::main]
async fn main() {
    localghost::log::Logger::new().start();
    let count = RefCell::new(0usize);
    let body = localghost::document().body();

    println!("suppp");
    localghost::log::info!("{}", "hjello");
    let button = Element::with_text(ElementKind::Button, "+ss");
    let count_handle = count.clone();
    button.on("click", move |_| {
        // let mut handle = count_handle.borrow_mut();
        // *handle += 1;
        localghost::log::info!("{}", "hjello");
        // let counter = query_selector("counter").unwrap_throw();
        // counter.set_text_content(Some(&format!("{}", handle)))
    });
    body.append_child(button);


    // let p = Element::with_text(ElementKind::P, &format!("{}", count.borrow()));
    // p.set_attribute("id", "counter");
    // body.append_child(p);

    // let button = Element::with_text(ElementKind::Button, "-");
    // let count_handle = count.clone();
    // button.on("click", move |_| {
    //     let mut handle = count_handle.borrow_mut();
    //     if *handle > 0 {
    //         *handle -= 1;
    //         let counter = query_selector("counter").unwrap_throw();
    //         counter.set_text_content(Some(&format!("{}", handle)))
    //     }
    // });
    // body.append_child(button);
}
