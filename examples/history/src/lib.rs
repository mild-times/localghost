use localghost::prelude::*;

use localghost::dom::{query_selector, self, Element, ElementKind};
use localghost::log;
use localghost::task;
use localghost::History;

use std::io;

#[localghost::main]
async fn main() -> io::Result<()> {
    let body = dom::body();

    let button = Element::with_text(ElementKind::Button, "back");
    button
        .on("click", move |_| {
            task::spawn_local(async {
                History::new().pop().await;
                log::info("history popped");
            });
        })
        .forget();
    body.append_child(button);

    let button = Element::with_text(ElementKind::Button, "hello");
    button
        .on("click", move |_| {
            task::spawn_local(async {
                History::new().push("/hello").await;
                log::info("history pushed");
            });
        })
        .forget();
    body.append_child(button);

    let button = Element::with_text(ElementKind::Button, "goodbye");
    button
        .on("click", move |_| {
            task::spawn_local(async {
                History::new().push("/goodbye").await;
                log::info("history pushed");
            });
        })
        .forget();
    body.append_child(button);
    Ok(())
}
