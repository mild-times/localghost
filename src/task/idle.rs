use futures_channel::oneshot::{channel, Receiver};
use futures_core::Stream;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::fmt::{self, Debug};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Perform work during a browser's idle period.
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
///
/// # Example
///
/// ```no_run
/// use localghost::task::Idle;
/// use localghost::prelude::*;
/// use async_std::prelude::*;
///
/// #[localghost::main]
/// async fn main() {
///     // Create an animator that will loop for 60 frames.
///     let mut animator = Idle::new().take(60);
///
///     let mut counter = 0;
///     while let Some(frame) = animator.next().await {
///         println!("frame no {}", counter);
///         counter += 1;
///     }
/// }
/// ```
pub struct Idle {
    receiver: Option<Receiver<()>>,
    f: Option<Closure<dyn std::ops::FnMut()>>,
    id: Option<u32>,
}

impl Idle {
    /// Create a new instance of `Idle`.
    pub fn new() -> Self {
        Self {
            receiver: None,
            id: None,
            f: None,
        }
    }
}

impl Stream for Idle {
    type Item = ();
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.receiver.is_none() {
            let window = crate::utils::window();
            let (sender, receiver) = channel();
            let f = Closure::once(move || sender.send(()).unwrap_throw());
            let id = window
                .request_idle_callback(f.as_ref().unchecked_ref())
                .unwrap_throw();

            // store the closure so it isn't dropped.
            self.f = Some(f);
            self.id = Some(id);
            self.receiver = Some(receiver);
        }

        match Pin::new(self.receiver.as_mut().unwrap()).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(_)) => {
                self.receiver = None;
                self.f = None;
                self.id = None;
                Poll::Ready(Some(()))
            }
            Poll::Ready(Err(_)) => panic!("error in Idle"),
        }
    }
}

impl Default for Idle {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Idle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Idle").finish()
    }
}

impl Drop for Idle {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            crate::utils::window().cancel_idle_callback(id);
        }
    }
}
