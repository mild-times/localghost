//! API bindings for Web APIs
//!
//! # Examples
//!
//!
//! ```no_run
//! use wasm_bindgen::prelude::*;
//! use coast::ready;
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     coast::task::spawn_local(async {
//!         println!("waiting on document to load");
//!         ready().await;
//!         println!("document loaded!");
//!     })
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
