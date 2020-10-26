//! The browser History API.
//!
//! Navigate back and forth through the user's history, and manipulate the contents of the history stack
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/API/History).
//!
//! Notes: https://yoshuawuyts-old.netlify.com/history

use crate::prelude::*;

use futures_channel::oneshot::channel;
use wasm_bindgen::JsValue;

// use std::cell::RefCell;
// use std::sync::atomic::{AtomicBool, Ordering};

// const EXISTS: RefCell<AtomicBool> = RefCell::new(AtomicBool::new(false));

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
/// We should reuse channels internally to improve performance, and on each
/// navigation we should pass a unique ID (probably some unique prefix + globally
/// incrementing number). That way we can associate an async fn with the right
/// corresponding location.
///
/// Probably to interop with other navigators, we should ensure that external
/// events store a location as well. If we see navigation event without a state,
/// we should assign it one. This can race though.
///
/// # References
///
/// - https://html.spec.whatwg.org/multipage/history.html#shared-history-push/replace-state-steps
/// - https://stackoverflow.com/questions/4570093/how-to-get-notified-about-changes-of-the-history-via-history-pushstate/4585031#4585031
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
        // assert_eq!(
        //     EXISTS
        //         .borrow()
        //         .compare_and_swap(false, true, Ordering::SeqCst),
        //     false,
        //     "Only one History instance can be created per application"
        // );

        Self {
            inner: crate::utils::window().history().unwrap_throw(),
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
    pub fn push(&self, url: &str) {
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
    pub async fn pop(&self) {
        let (sender, receiver) = channel();
        let _listener = crate::utils::window().once("popstate", move |_| {
            sender.send(()).unwrap();
        });
        self.inner.back().unwrap_throw();
        receiver.await.unwrap();
    }

    /// Replace the url currently on the stack with another url.
    ///
    /// # Cursor
    ///
    /// This keeps the cursor in-place.
    pub fn replace(&self, url: &str) {
        let null = JsValue::null();
        self.inner
            .replace_state_with_url(&null, "", Some(url))
            .unwrap_throw();
    }

    /// Move the cursor forward in time.
    ///
    /// # Cursor
    ///
    /// This moves the history cursor forward.
    pub async fn forward(&self) {
        let (sender, receiver) = channel();
        let _listener = crate::utils::window().once("popstate", move |_| {
            sender.send(()).unwrap();
        });
        self.inner.forward().unwrap_throw();
        receiver.await.unwrap();
    }

    // fn len
    // fn forward
    // fn backward
    // fn go
}

// impl Drop for History {
//     fn drop(&mut self) {
//         // Allow a new instance to be created once this no longer exists.
//         EXISTS.borrow().store(false, Ordering::SeqCst);
//     }
// }
// TODO: impl Stream & DoubleEndedStream for History
