use localghost::dom::{Element, ElementKind};
use localghost::prelude::*;
use localghost::net::Request;

use futures::stream::StreamExt;
use std::io;

#[localghost::main]
async fn main() -> io::Result<()> {
    let req = Request::new("get", "https://httpbin.org/status/200");
    let res = req.send().await?;
    localghost::log::info!("{}", res.status());
    localghost::log::info!("{:?}", res.body_bytes().await?);
    Ok(())
}
