//! The browser's history API
//!
//! Read more on [MDN - Web APIs: History](https://developer.mozilla.org/en-US/docs/Web/API/History).
//! Notes: https://yoshuawuyts-old.netlify.com/history

use crate::prelude::*;

use futures_channel::oneshot::channel;
use wasm_bindgen::JsValue;

/// The Web History API.
///
/// This provides a structured view onto the browser's history stack.
///
/// # Cursor
///
/// The state of the history API is kept in the browser's "history stack". We can
/// move forward, backward, and modify the stack freely from Rust. As such the
/// `History` type behaves itself as an `Iterator`, and is stateful.
///
/// The different methods on this type modify the history stack's cursor
/// differently. As such each method documents how it behaves.
///
/// # TODO
///
/// We should implement a new method, start, that tracks where in the history
/// stack this instance was created. That way it's possible to prevent going back
/// beyond the start of the history API.
///
/// Aditionally we should reuse channels internally to improve performance.
///
/// Also we should make use of real URL objects.
#[derive(Debug)]
pub struct History {
    inner: web_sys::History,
}

impl History {
    /// Create a new instance of `History`.
    ///
    /// # Cursor
    ///
    /// The cursor is initialized at the end of the stack.
    pub fn new() -> Self {
        Self {
            inner: crate::window().history().unwrap_throw(),
        }
    }

    /// Returns the number of history events in the stack.
    ///
    /// # Cursor
    ///
    /// The cursor does not move.
    pub fn len(&self) -> usize {
        self.inner.length().unwrap_throw() as usize
    }

    /// Push a new url onto the history stack.
    ///
    /// # Cursor
    ///
    /// This moves the cursor forward.
    pub fn push(&mut self, url: &str) {
        let null = JsValue::null();
        self.inner
            .push_state_with_url(&null, "", Some(url))
            .unwrap_throw();
    }

    /// Pop a url from the history stack.
    ///
    /// # Cursor
    ///
    /// This moves the history cursor backward.
    pub async fn pop(&mut self) {
        let (sender, receiver) = channel();
        let _listener = crate::window().once("popstate", move |_| {
            sender.send(()).unwrap();
        });
        self.inner.back().unwrap_throw();
        receiver.await.unwrap();
    }

    /// Pop a url from the history stack.
    ///
    /// # Cursor
    ///
    /// This moves the history cursor backward.
    ///
    /// # Stability
    ///
    /// This method will be removed in favor of a `DoubleEndedStream` impl later
    /// on.
    pub async fn forward(&mut self) {
        let (sender, receiver) = channel();
        let _listener = crate::window().once("popstate", move |_| {
            sender.send(()).unwrap();
        });
        self.inner.forward().unwrap_throw();
        receiver.await.unwrap();
    }

    /// Replace the url currently on the stack with another url.
    ///
    /// # Cursor
    ///
    /// This keeps the cursor in-place.
    pub fn replace(&mut self, url: &str) {
        let null = JsValue::null();
        self.inner
            .replace_state_with_url(&null, "", Some(url))
            .unwrap_throw();
    }
}

// TODO: impl Stream & DoubleEndedStream for History
