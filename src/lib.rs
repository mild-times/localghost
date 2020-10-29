//! <div align="center"><img width="730" alt="localghost logo" src="https://user-images.githubusercontent.com/2467194/97103696-5ddfca00-16ae-11eb-9864-3a2c21188555.png"></div>
//!
//! Safe Rust bindings to the Web platform.
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
//!     log::info!("responded with {:?}", res.status());
//!     Ok(())
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
