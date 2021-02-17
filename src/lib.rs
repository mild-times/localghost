//! <div align="center"><img width="730" alt="localghost logo" src="https://user-images.githubusercontent.com/2467194/97103696-5ddfca00-16ae-11eb-9864-3a2c21188555.png"></div>
//!
//! # Ergonomic Rust bindings to the Web platform.
//!
//! This crate builds on [wasm-bindgen](https://docs.rs/wasm-bindgen),
//! [js-sys](https://docs.rs/js-sys), and [web-sys](https://docs.rs/web-sys) to
//! provide high-level ergonomic bindings to all Web platform APIs. The goal is
//! to empower library and framework authors to bring Rust to the web, being able
//! to use familiar idioms exposed through a carefully designed API.
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
//!     let res = net::Request::get("https://httpbin.org/uuid").send().await?;
//!     log::info!("status: {:?}", res.status());
//!     log::info!("body: {:?}", res.body_string().await?);
//!     Ok(())
//! }
//! ```
//!
//! For more examples see the [examples] directory.
//!
//! [examples]: https://github.com/yoshuawuyts/mild/tree/master/examples
//!
//! # Getting started
//!
//! In order to build a `localghost` project you need
//! [wasm-pack](https://docs.rs/wasm-pack/0.9.1/wasm_pack/) installed, and a
//! fresh Rust project with a `lib.rs` file. Then add the following to your `Cargo.toml`:
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! localghost = "0.1.0"
//! ```
//!
//! You can then compile the project using `wasm-pack build --target web` and
//! serve it from an HTML file using the following snippet:
//!
//! ```html
//! <body>
//!   <script type="module">
//!     import init from "./pkg/<project_name>.js";
//!     init("./pkg/<project_name>.wasm");
//!   </script>
//! </body>
//! ```

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub, rust_2018_idioms)]
#![allow(clippy::module_inception)]

pub mod dom;
pub mod events;
pub mod fs;
pub mod keyboard;
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
