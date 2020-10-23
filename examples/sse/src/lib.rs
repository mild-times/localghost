// use localghost::dom::{query_selector, Element, ElementKind};
use localghost::prelude::*;
use localghost::net;

use futures::stream::StreamExt;

#[localghost::main]
async fn main() {
    let mut sse = net::EventSource::connect("http://localhost:8081/sse").await.unwrap_throw();
    sse.register("fruit");

    while let Some(msg) = sse.next().await {
        localghost::log::info!("{:?}", msg.unwrap_throw());
    }
}
