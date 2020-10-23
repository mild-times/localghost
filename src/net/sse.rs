use crate::events::EventListener;
use crate::prelude::*;
use crate::utils::ResultExt;

use async_channel::{bounded, Receiver};
use std::io;

/// The state of the SSE connection.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ReadyState {
    /// The connection is connecting.
    Connecting,
    /// The connection is open.
    Open,
    /// The connnection is closed.
    Closed,
}
/// An SSE event with a data payload.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MessageEvent {
    /// The ID of this event.
    ///
    /// See also the [Server-Sent Events spec](https://html.spec.whatwg.org/multipage/server-sent-events.html#concept-event-stream-last-event-id).
    pub(crate) id: Option<String>,
    /// The event name. Defaults to "message" if no event name is provided.
    pub(crate) name: String,
    /// The data for this event.
    pub(crate) data: Vec<u8>,
}

impl MessageEvent {
    /// Get the message id.
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    /// Get the message event name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Access the event data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Convert the message into the data payload.
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }
}

/// A receiver of `Server Sent Events` (SSE).
#[derive(Debug)]
pub struct EventSource {
    /// The internal `EventSource` handle.
    inner: web_sys::EventSource,
    /// The url we connect to.
    url: String,
    /// A listener that catches errors from the stream.
    err_receiver: Receiver<io::Error>,
    /// track whether the listener is still open
    open: bool,
}

impl EventSource {
    /// Create a new instance of `EventSource` and wait for a connection to be
    /// established.
    pub async fn connect(url: &str) -> io::Result<Self> {
        let url = url.to_owned();
        crate::log::debug!("EventSource({}): connection started", url);
        let inner = web_sys::EventSource::new(&url).err_kind(io::ErrorKind::InvalidInput)?;

        // Add an error listener that will store exactly 1 error.
        let url2 = url.clone();
        let (sender, err_receiver) = bounded::<io::Error>(1);
        inner.on("error", move |_| {
            crate::log::debug!("EventSource({}): connection error", url2);
            let err = io::Error::new(
                io::ErrorKind::Other,
                format!("EventSource({}) connection error", url2).as_str(),
            );
            let _ = sender.try_send(err);
        });

        // Wait to open.
        let (sender, receiver) = bounded(1);
        let listener =
            EventListener::listen_once(&inner, "open", move |_| sender.try_send(()).unwrap_throw());
        receiver.recv().await.unwrap_throw();
        drop(listener);

        crate::log::debug!("EventSource({}): connection established", url);

        let mut this = Self {
            url,
            inner,
            err_receiver,
            open: true,
        };

        // Check no errors have occurred.
        this.check_err()?;
        Ok(this)
    }

    /// Access the `EventSource`'s connection state.
    pub fn ready_state(&self) -> ReadyState {
        match self.inner.ready_state() {
            web_sys::EventSource::CONNECTING => ReadyState::Connecting,
            web_sys::EventSource::OPEN => ReadyState::Open,
            web_sys::EventSource::CLOSED => ReadyState::Closed,
            _ => unreachable!("Unknown EventSource ready state"),
        }
    }

    /// Close the inner connection. This is called on error and `Drop`.
    fn close(&mut self) {
        if self.open {
            self.open = false;
            self.inner.close();
            crate::log::debug!("EventSource({}): instance closed", self.url);
        }
    }

    /// Check whether an error has been sent into the channel.
    fn check_err(&mut self) -> io::Result<()> {
        match self.err_receiver.try_recv() {
            Ok(err) => {
                self.close();
                Err(err)
            }
            _ => Ok(()),
        }
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        self.close();
    }
}

impl AsRef<web_sys::EventTarget> for EventSource {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}
