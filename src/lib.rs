//! API bindings for Web APIs
//!
//! # Examples
//!
//!
//! ```no_run
//! use coast::prelude::*;
//! use coast::{ready, net};
//!
//! #[coast::main]
//! async fn main() {
//!     ready().await;
//!     let res = net::Request::new("GET", "https://example.com").send().await;
//!     println!("responded with {:?}", res.status_code());
//! }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

pub mod events;
pub mod log;
pub mod net;
pub mod prelude;
pub mod task;

mod document_ready;
mod history;
mod window;

pub use document_ready::ready;
pub use history::History;
pub use window::window;
