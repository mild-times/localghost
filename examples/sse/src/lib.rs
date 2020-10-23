// use localghost::dom::{query_selector, Element, ElementKind};
use localghost::prelude::*;
use localghost::net;

#[localghost::main]
async fn main() {
    localghost::log::info!("hiiiiii");
    let sse = net::EventSource::connect("http://localhost:8081/sse").await.unwrap_throw();
    sse.on("fruit", |ev| {
        localghost::log::info!("{:?}", ev);
    });
}
