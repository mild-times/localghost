#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use crate::events::EventTarget;
use crate::prelude::*;

use futures_channel::oneshot::channel;

use std::time::Duration;

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
        "complete" | "interactive" => {
            futures_timer::Delay::new(Duration::from_secs(0)).await;
        }
        _ => {
            let (sender, receiver) = channel();
            let _listener = document.once("DOMContentLoaded", move |_| {
                sender.send(()).unwrap();
            });
            receiver.await.unwrap();
        }
    };
}
