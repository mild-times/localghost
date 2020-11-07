use localghost::prelude::*;

use localghost::dom::{query_selector, self, Element};
use localghost::log;
use localghost::task;
use localghost::History;

use std::io;

#[localghost::main]
async fn main() -> io::Result<()> {
    let body = dom::body();

    let button = Element::with_text("button", "back");
    button
        .on_with("click", move |_| {
            task::spawn_local(async {
                History::new().pop().await;
                log::info("history popped");
            });
        })
        .forget();
    body.append(button);

    let button = Element::with_text("button", "hello");
    button
        .on_with("click", move |_| {
            task::spawn_local(async {
                History::new().push("/hello").await;
                log::info("history pushed");
            });
        })
        .forget();
    body.append(button);

    let button = Element::with_text("button", "goodbye");
    button
        .on_with("click", move |_| {
            task::spawn_local(async {
                History::new().push("/goodbye").await;
                log::info("history pushed");
            });
        })
        .forget();
    body.append(button);
    Ok(())
}
