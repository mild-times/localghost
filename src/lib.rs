//! API bindings for Web APIs
//!
//! # Examples
//!
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::{log, net};
//!
//! #[localghost::main]
//! async fn main() {
//!     let res = net::Request::new("GET", "https://example.com").send().await.unwrap();
//!     log::info!("responded with {:?}", res.status_code());
//! }
//! ```

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(
    missing_docs,
    missing_doc_code_examples,
    unreachable_pub,
    rust_2018_idioms
)]

pub mod dom;
pub mod events;
pub mod log;
pub mod net;
pub mod prelude;
pub mod task;

mod history;

#[doc(inline)]
pub use dom::{document, window};
pub use history::History;

#[doc(inline)]
pub use localghost_macros::main;

#[doc(hidden)]
pub mod macro_export {
    pub use console_error_panic_hook::set_once as set_panic_hook;
}

/// Raw bindings to JS and the DOM.
pub mod raw {
    pub use js_sys;
    pub use wasm_bindgen;
    pub use web_sys;
}
