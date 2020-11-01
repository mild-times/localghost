//! Browser keyboard API
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
//!
//! # Examples
//!
//! ```no_run
//! use localghost::dom::{self, Element, ElementKind};
//! use localghost::prelude::*;
//! use localghost::keyboard::Keyboard;
//!
//! #[localghost::main]
//! async fn main() {
//!     // Connect the `Keyboard`.
//!     let keyboard = Keyboard::new();
//!
//!     // Create a table
//!     let table = Element::new(ElementKind::Table);
//!     dom::body().append_child(&table);
//!
//!     // Create the headings
//!     let tr = Element::new(ElementKind::Tr);
//!     tr.append_child(Element::with_text(ElementKind::Th, "key name"));
//!     table.append_child(tr);
//!
//!     // For every keyboard event add an entry to the table.
//!     let mut keydown = keyboard.keydown();
//!     while let Some(ev) = keydown.next().await {
//!         let tr = Element::new(ElementKind::Tr);
//!         tr.append_child(Element::with_text(ElementKind::Td, &ev.key()));
//!         table.append_child(tr);
//!     };
//! }
//! ```
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

pub use modifier_key::ModifierKey;

mod modifier_key;

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

/// A keyboard event.
#[derive(Debug)]
pub struct KeyboardEvent {
    inner: web_sys::KeyboardEvent,
}

impl KeyboardEvent {
    /// Returns the value of the key pressed by the user, taking into
    /// consideration the state of modifier keys such as Shift as well as the
    /// keyboard locale and layout.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    pub fn key(&self) -> KeyKind {
        let key = self.inner.key();
        match key.as_str() {
            "Unidentified" => KeyKind::Unidentified,
            "Dead" => KeyKind::Dead,
            _ => KeyKind::Key(key),
        }
    }

    /// Indicates if the `alt` key (`Option` or `⌥` on macOS) was pressed when
    /// the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.inner.alt_key()
    }

    /// Indicates if the `control` key was pressed when the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)
    pub fn ctrl_key(&self) -> bool {
        self.inner.ctrl_key()
    }

    /// Indicates if the `shift` key was pressed when the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)
    pub fn shift_key(&self) -> bool {
        self.inner.shift_key()
    }

    /// Indicates whether the specified key was pressed or locked when the event
    /// occurred.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)
    pub fn modifier_key(&self, modifier: ModifierKey) -> bool {
        self.inner.get_modifier_state(modifier.as_str())
    }

    /// Indicates if the event is fired within a composition session.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)
    pub fn is_composing(&self) -> bool {
        self.inner.is_composing()
    }

    /// Indicates if the given key is being held down such that it is automatically repeating.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
    pub fn is_repeating(&self) -> bool {
        self.inner.repeat()
    }

    // TODO: location, init an enum -- https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    // TODO: key_code, init an enum -- https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
}

/// The computed `key` value of a `KeyboardEvent`
///
/// This `struct` is created by the [`key`] method on [`KeyboardEvent`]. See its
/// documentation for more.
///
/// [`key`]: struct.KeyboardEvent.html#method.key
/// [`KeyboardEvent`]: struct.KeyboardEvent.html
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KeyKind {
    /// The printable representation of a key.
    Key(String),
    /// The value of the key could not be identified.
    Unidentified,
    /// The key is considered a ["dead key"](https://en.wikipedia.org/wiki/Dead_key).
    Dead,
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
