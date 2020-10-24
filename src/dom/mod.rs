//! The browser's Document Object Model.
//!
//! # Examples
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::dom::{Element, ElementKind, Text};
//!
//! #[localghost::main]
//! async fn main() {
//!     let p = Element::with_text(ElementKind::P, "Hello world");
//!     let body = localghost::document().body();
//!     body.append_child(p);
//! }
//! ```

// re-exports, temporary only
pub use document::Document;
pub use element::Element;
pub use element_kind::ElementKind;
pub use query_selector::query_selector;
pub use text::Text;
pub use window::Window;

mod document;
mod element;
mod element_kind;
mod query_selector;
mod text;
mod window;
