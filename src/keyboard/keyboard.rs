use std::pin::Pin;
use std::task::{Context, Poll};

use async_std::stream::Stream;
use async_std::task;
use pin_project::pin_project;
use wasm_bindgen::JsCast;

use crate::events::EventStream;
use crate::keyboard::KeyboardEvent;
use crate::prelude::*;
use crate::utils;

/// Browser keyboard API.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
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
    pub fn key_down(&self) -> KeyDownStream {
        KeyDownStream {
            listener: utils::document().on("keydown"),
        }
    }

    /// Create a stream for [`keyup`
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event).
    pub fn key_up(&self) -> KeyDownStream {
        KeyDownStream {
            listener: utils::document().on("keyup"),
        }
    }
}

/// A stream capturing `keydown` events.
///
/// This `struct` is created by the [`key_down`] method on [`Keyboard`]. See its
/// documentation for more.
///
/// [`key_down`]: struct.Keyboard.html#method.key_down
/// [`Keyboard`]: struct.Keyboard.html
#[pin_project]
#[derive(Debug)]
pub struct KeyDownStream {
    #[pin]
    listener: EventStream,
}

impl Stream for KeyDownStream {
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
/// This `struct` is created by the [`key_up`] method on [`Keyboard`]. See its
/// documentation for more.
///
/// [`key_up`]: struct.Keyboard.html#method.key_up
/// [`Keyboard`]: struct.Keyboard.html
#[pin_project]
#[derive(Debug)]
pub struct KeyUpStream {
    #[pin]
    listener: EventStream,
}

impl Stream for KeyUpStream {
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
