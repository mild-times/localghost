//! <div align="center"><img width="730" alt="localghost logo" src="https://user-images.githubusercontent.com/2467194/97103696-5ddfca00-16ae-11eb-9864-3a2c21188555.png"></div>
//!
//! Safe Rust bindings to the Web platform.
//!
//! # Examples
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
#![warn(missing_docs, unreachable_pub, rust_2018_idioms)]

pub mod dom;
pub mod events;
pub mod fs;
pub mod log;
pub mod net;
pub mod prelude;
pub mod task;

mod history;
mod location;
mod utils;

pub use history::History;
#[doc(inline)]
pub use localghost_macros::main;
pub use location::Location;

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

/// Access the browser's `Document` object.
///
/// # Errors
///
/// This function panics if a `Document` is not found.
///
/// # Example
///
/// ```no_run
/// let doc = localghost::document();
/// # drop(doc)
/// ```
pub fn document() -> dom::Document {
    dom::Document::new()
}

/// Access the browser's `Window` object.
///
/// # Errors
///
/// This function panics if a `Window` is not found.
///
/// # Example
///
/// ```no_run
/// let window = localghost::window();
/// # drop(window)
/// ```
pub fn window() -> dom::Window {
    dom::Window::new()
}
