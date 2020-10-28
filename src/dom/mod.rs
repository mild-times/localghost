//! The browser's Document Object Model.
//!
//! # Examples
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::dom::{self, Element, ElementKind};
//!
//! #[localghost::main]
//! async fn main() {
//!     let p = Element::with_text(ElementKind::P, "Hello world");
//!     dom::body().append_child(p);
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

use crate::prelude::*;

/// Get the document's body.
pub fn body() -> Element {
    let el = crate::utils::document()
        .query_selector("body")
        .expect_throw("Could not find `window.body`")
        .expect_throw("Could not find `window.body`");
    unsafe { Element::from_raw(ElementKind::Body, el) }
}
