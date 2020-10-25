use localghost::prelude::*;
use localghost::net::Request;
use localghost::log;

use std::io;

#[localghost::main]
async fn main() -> io::Result<()> {
    let res = Request::new("get", "https://httpbin.org/uuid").send().await?;
    log::info!("{:?}", res.body_len());
    log::info!("{}, {}", res.status(), res.body_string().await?);
    Ok(())
}
