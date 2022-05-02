use futures_channel::oneshot::{channel, Receiver};
use futures_core::Stream;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::IdleDeadline;
use web_sys::IdleRequestOptions;

use std::fmt::{self, Debug};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

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
/// use std::time::Duration;
///
/// #[localghost::main]
/// async fn main() {
///     // Create an idle loop which will trigger 60 times.
///     let mut idle = Idle::new().take(60);
///
///     let mut counter = 0;
///     while let Some(deadline) = idle.next().await {
///         println!("frame no {}", counter);
///         assert!(deadline.time_remaining() >= Duration::ZERO);
///         counter += 1;
///     }
/// }
/// ```
pub struct Idle {
    options: Option<IdleRequestOptions>,
    receiver: Option<Receiver<IdleDeadline>>,
    f: Option<Closure<dyn std::ops::FnMut(IdleDeadline)>>,
    id: Option<u32>,
}

impl Idle {
    /// Create a new instance of `Idle`.
    pub fn new() -> Self {
        Self {
            options: None,
            receiver: None,
            id: None,
            f: None,
        }
    }

    /// Create a new instance of `Idle` with an optional deadline.
    ///
    /// Setting a deadline ensures that if the browser doesn't have any idle
    /// time before the deadline elapses, the idle loop will still trigger.
    pub fn with_deadline(deadline: Duration) -> Self {
        let mut options = IdleRequestOptions::new();
        options.timeout(deadline.as_millis() as u32);
        Self {
            options: Some(options),
            receiver: None,
            id: None,
            f: None,
        }
    }
}

impl Stream for Idle {
    type Item = Deadline;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.receiver.is_none() {
            let window = crate::utils::window();
            let (sender, receiver) = channel();
            let f = Closure::once(move |deadline| sender.send(deadline).unwrap_throw());
            let id = match &self.options {
                Some(options) => {
                    let callback = &f.as_ref().unchecked_ref();
                    window.request_idle_callback_with_options(callback, options)
                }
                None => {
                    let callback = &f.as_ref().unchecked_ref();
                    window.request_idle_callback(callback)
                }
            }
            .unwrap_throw();

            // store the closure so it isn't dropped.
            self.f = Some(f);
            self.id = Some(id);
            self.receiver = Some(receiver);
        }

        match Pin::new(self.receiver.as_mut().unwrap()).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(deadline)) => {
                self.receiver = None;
                self.f = None;
                self.id = None;
                let deadline = Deadline { inner: deadline };
                Poll::Ready(Some(deadline))
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

/// A deadline yielded by [`Idle`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deadline {
    inner: IdleDeadline,
}

impl Deadline {
    /// Estimate how much time remains.
    pub fn time_remaining(&self) -> Duration {
        // Convert from milliseconds f64 to secs f64.
        let remaining = self.inner.time_remaining() * 1000.0;
        Duration::from_secs_f64(remaining)
    }

    /// Returns `true` if the event was triggered
    /// because of a timeout.
    pub fn did_timeout(&self) -> bool {
        self.inner.did_timeout()
    }
}
