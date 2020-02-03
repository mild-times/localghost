//! API bindings for Web APIs
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

pub mod events;
pub mod task;

mod document_ready;
mod window;

#[doc(inline)]
pub use web_sys as sys;
pub use document_ready::ready;
pub use window::window;
