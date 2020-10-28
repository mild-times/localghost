use crate::dom::{Element, ElementKind};
use crate::events::EventTarget;
use crate::prelude::*;

use std::ops::{Deref, DerefMut};

/// A reference to the root document.
#[derive(Debug)]
pub struct Document {
    doc: web_sys::Document,
}

impl Document {
    /// Create a new `Document`.
    pub fn new() -> Self {
        let doc = crate::utils::window()
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
            _ => self.once("DOMContentLoaded").await,
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
