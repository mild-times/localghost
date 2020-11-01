//! Browser networking APIs
//!
//! # Examples
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::{log, net};
//! use std::io;
//!
//! #[localghost::main]
//! async fn main() -> io::Result<()> {
//!     let res = net::Request::get("https://example.com").send().await?;
//!     log::info!("status: {:?}", res.status());
//!     log::info!("body: {:?}", res.body_string().await?);
//!     Ok(())
//! }
//! ```

mod beacon;
mod body;
mod event_source;
mod headers;
mod message_event;
mod request;
mod response;

use headers::Headers;

pub use beacon::Beacon;
pub use body::Body;
pub use event_source::{EventSource, ReadyState};
pub use headers::HeadersIter;
pub use message_event::MessageEvent;
pub use request::Request;
pub use response::Response;
