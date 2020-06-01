use crate::events::EventTarget;
use crate::prelude::*;
use crate::window;

use std::ops::{Deref, DerefMut};

use futures_channel::oneshot::channel;

/// Access the browser's `Document` object.
///
/// # Errors
///
/// This function panics if a `Document` is not found.
///
/// # Example
///
/// ```no_run
/// let doc = coast::document();
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

/// Wait for the DOM to be loaded.
///
/// # Examples
///
/// ```no_run
/// use wasm_bindgen::prelude::*;
/// use coast::ready;
///
/// #[wasm_bindgen(start)]
/// pub fn main() {
///     coast::task::spawn_local(async {
///         println!("waiting on document to load");
///         ready().await;
///         println!("document loaded!");
///     })
/// }
/// ```

pub async fn ready() {
    let document = crate::window().document().unwrap_throw();

    match document.ready_state().as_str() {
        "complete" | "interactive" => return,
        _ => {
            let (sender, receiver) = channel();
            let _listener = document.once("DOMContentLoaded", move |_| {
                sender.send(()).unwrap();
            });
            receiver.await.unwrap();
        }
    };
}
