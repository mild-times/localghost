//! The root element of the web page.

use crate::events::EventTarget;
use crate::prelude::*;

use futures_channel::oneshot::channel;
use std::ops::{Deref, DerefMut};

// re-exports, temporary only
pub use element::Element;
pub use element_kind::ElementKind;
pub use text::Text;
pub use window::{window, Window};

mod element;
mod element_kind;
mod text;
mod window;

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
pub fn document() -> Document {
    Document::new()
}

/// A reference to the root document.
#[derive(Debug)]
pub struct Document {
    doc: web_sys::Document,
}

impl Document {
    /// Create a new `Document`.
    pub fn new() -> Self {
        let doc = window()
            .document()
            .expect_throw("Could not find `window.document`");
        Self { doc }
    }

    /// Get the Body from the document.
    pub fn body(&self) -> Element {
        let el = self
            .doc
            .query_selector("body")
            .expect_throw("Could not find `window.body`")
            .expect_throw("Could not find `window.body`");
        unsafe { Element::from_raw(ElementKind::Body, el) }
    }

    /// Wait for the DOM to be loaded.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wasm_bindgen::prelude::*;
    /// use localghost::ready;
    ///
    /// #[wasm_bindgen(start)]
    /// pub fn main() {
    ///     localghost::task::spawn_local(async {
    ///         println!("waiting on document to load");
    ///         ready().await;
    ///         println!("document loaded!");
    ///     })
    /// }
    /// ```
    pub async fn ready(&self) {
        match self.ready_state().as_str() {
            "complete" | "interactive" => return,
            _ => {
                let (sender, receiver) = channel();
                let _listener = self.once("DOMContentLoaded", move |_| {
                    sender.send(()).unwrap();
                });
                receiver.await.unwrap();
            }
        };
    }
}

impl Deref for Document {
    type Target = web_sys::Document;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.doc
    }
}
