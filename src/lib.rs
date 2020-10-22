//! API bindings for Web APIs
//!
//! # Examples
//!
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::{ready, net};
//!
//! #[localghost::main]
//! async fn main() {
//!     ready().await;
//!     let res = net::Request::new("GET", "https://example.com").send().await;
//!     println!("responded with {:?}", res.status_code());
//! }
//! ```

#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

pub mod document;
pub mod events;
pub mod log;
pub mod net;
pub mod prelude;
pub mod task;

mod history;
mod window;

#[doc(inline)]
pub use document::{document, Document};
pub use history::History;
pub use window::{window, Window};

#[doc(inline)]
pub use localghost_macros::main;
