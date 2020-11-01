//! Browser keyboard API
// - Stream to capture keydown events
// - Stream to capture keyup events
// - lock API -> KeyboardLock
// - synthetic keyboard event
//    - create a new event
//    - emit the event from the window

use std::pin::Pin;
use std::task::{Context, Poll};

use async_std::stream::Stream;
use async_std::task;
use pin_project::pin_project;
use wasm_bindgen::JsCast;

use crate::events::EventStream;
use crate::prelude::*;
use crate::utils;

/// Browser keyboard API.
#[derive(Debug)]
pub struct Keyboard {
    _priv: (),
}

impl Keyboard {
    /// Create a new instance of `Keyboard`.
    pub fn new() -> Self {
        Self { _priv: () }
    }

    /// Create a stream for [`keydown`
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event).
    pub fn keydown(&self) -> KeydownStream {
        KeydownStream {
            listener: utils::document().on("keydown"),
        }
    }

    /// Create a stream for [`keyup`
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event).
    pub fn keyup(&self) -> KeydownStream {
        KeydownStream {
            listener: utils::document().on("keyup"),
        }
    }
}

/// A keyboard event.
#[derive(Debug)]
pub struct KeyboardEvent {
    inner: web_sys::KeyboardEvent,
}

impl KeyboardEvent {
    /// Get the key.
    pub fn key(&self) -> String {
        self.inner.key()
    }
}

/// A stream capturing `keydown` events.
///
/// This `struct` is created by the [`keydown`] method on [`Keyboard`]. See its
/// documentation for more.
///
/// [`keydown`]: struct.Keyboard.html#method.keydown
/// [`Keyboard`]: struct.Keyboard.html
#[pin_project]
#[derive(Debug)]
pub struct KeydownStream {
    #[pin]
    listener: EventStream,
}

impl Stream for KeydownStream {
    type Item = KeyboardEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let ev = task::ready!(this.listener.poll_next(cx));
        let ev = ev.map(|ev| {
            let inner = ev
                .into_raw()
                .dyn_into::<web_sys::KeyboardEvent>()
                .unwrap_throw();
            KeyboardEvent { inner }
        });
        Poll::Ready(ev)
    }
}

/// A stream capturing `keyup` events.
///
/// This `struct` is created by the [`keyup`] method on [`Keyboard`]. See its
/// documentation for more.
///
/// [`keyup`]: struct.Keyboard.html#method.keyup
/// [`Keyboard`]: struct.Keyboard.html
#[pin_project]
#[derive(Debug)]
pub struct KeyupStream {
    #[pin]
    listener: EventStream,
}

impl Stream for KeyupStream {
    type Item = KeyboardEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let ev = task::ready!(this.listener.poll_next(cx));
        let ev = ev.map(|ev| {
            let inner = ev
                .into_raw()
                .dyn_into::<web_sys::KeyboardEvent>()
                .unwrap_throw();
            KeyboardEvent { inner }
        });
        Poll::Ready(ev)
    }
}
