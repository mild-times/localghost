//! The browser History API.
//!
//! Navigate back and forth through the user's history, and manipulate the contents of the history stack
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/API/History).
//!
//! Notes: https://yoshuawuyts-old.netlify.com/history

use std::pin::Pin;
use std::task::{Context, Poll};

use crate::prelude::*;

use futures_core::Stream;
use pin_project::pin_project;
use wasm_bindgen::JsValue;

/// The Web History API.
///
/// The History API represents the URL and forward/backward buttons in the
/// browser. It's modeled as a stack where URLs can be pushed/popped, and a
/// cursor can be moved forward/backward along the stack.
///
/// `History` also implements `Stream` which returns a url every time the
/// cursor is moved along the stack. The only time this event is not triggered is
/// when the cursor moves because of a new entry being pushed onto the stack.
#[pin_project]
#[derive(Debug)]
pub struct History {
    inner: web_sys::History,
    #[pin]
    listener: Option<crate::events::EventStream>,
}

impl History {
    /// Create a new instance of `History`.
    pub fn new() -> Self {
        Self {
            inner: crate::utils::window().history().unwrap_throw(),
            listener: None,
        }
    }

    /// Returns the number of urls in the History stack.
    pub fn len(&self) -> usize {
        self.inner.length().unwrap_throw() as usize
    }

    /// Returns `true` if the History stack contains no items.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the current url from the History stack.
    pub fn current(&self) -> String {
        crate::utils::document()
            .location()
            .unwrap_throw()
            .href()
            .unwrap_throw()
    }

    /// Push a new url onto the history stack.
    pub fn push(&self, url: &str) {
        let null = JsValue::null();
        self.inner
            .push_state_with_url(&null, "", Some(url))
            .unwrap_throw();
    }

    /// Pop a url from the history stack.
    pub fn pop(&self) {
        self.inner.back().unwrap_throw();
    }

    /// Replace the url currently on the stack with another url.
    pub fn replace(&self, url: &str) {
        let null = JsValue::null();
        self.inner
            .replace_state_with_url(&null, "", Some(url))
            .unwrap_throw();
    }

    /// Move the cursor forwards and backwards through the History stack
    /// depending on the value of the parameter.
    ///
    /// Passing a value of `0` will reload the page.
    pub async fn move_by(&self, delta: i32) {
        let fut = crate::utils::window().once("popstate");
        self.inner.go_with_delta(delta).unwrap_throw();
        fut.await;
    }

    /// Moves the cursor to the next url in the History stack.
    pub async fn move_next(&self) {
        let fut = crate::utils::window().once("popstate");
        self.inner.forward().unwrap_throw();
        fut.await;
    }

    /// Moves the cursor to the previous url in the History stack.
    pub async fn move_prev(&self) {
        let fut = crate::utils::window().once("popstate");
        self.inner.back().unwrap_throw();
        fut.await;
    }

    /// Reload the current url, like the Refresh button.
    pub async fn reload(&self) {
        let fut = crate::utils::window().once("popstate");
        self.inner.go().unwrap_throw();
        fut.await;
    }
}

impl Stream for History {
    type Item = String;

    /// Return URLs generated by moving the cursor on the stack. This is
    /// equivalent to listening for the `"popstate"` event.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.listener.is_none() {
            self.listener = Some(crate::utils::window().on("popstate"));
        }

        let listener = self.project().listener.get_mut().as_mut().unwrap_throw();
        match Pin::new(listener).poll_next(cx) {
            Poll::Ready(_) => Poll::Ready(Some(
                crate::utils::document()
                    .location()
                    .unwrap_throw()
                    .href()
                    .unwrap_throw(),
            )),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
