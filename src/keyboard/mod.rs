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
//! use futures::stream::StreamExt;
//!
//! #[localghost::main]
//! async fn main() {
//!     let keyboard = Keyboard::new();
//!     let body = dom::body();
//!
//!     let desc = Element::with_text(ElementKind::P, "Press a key, get a key name");
//!     body.append(desc);
//!
//!     let heading = Element::new(ElementKind::H1);
//!     heading.set_attr("id", "target");
//!     body.append(heading);
//!
//!     // For every keyboard event modify the heading.
//!     let mut keydown = keyboard.key_down();
//!     while let Some(ev) = keydown.next().await {
//!         let el = dom::query_selector("#target").unwrap_throw();
//!         el.set_text(&ev.key().to_string());
//!     };
//! }
//! ```
// - Stream to capture keydown events
// - Stream to capture keyup events
// - lock API -> KeyboardLock
// - synthetic keyboard event
//    - create a new event
//    - emit the event from the window

pub use key_kind::KeyKind;
pub use keyboard::{KeyDownStream, KeyUpStream, Keyboard};
pub use keyboard_event::KeyboardEvent;
pub use modifier_key::ModifierKey;

mod key_kind;
mod keyboard;
mod keyboard_event;
mod modifier_key;
