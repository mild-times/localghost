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

/// A receiver of `Server Sent Events` (SSE).
#[derive(Debug)]
pub struct EventSource {
    /// The internal `EventSource` handle.
    inner: web_sys::EventSource,
    /// A listener that catches errors from the stream.
    err_receiver: Receiver<io::Error>,
}

impl EventSource {
    /// Create a new instance of `EventSource` and wait for a connection to be
    /// established.
    pub async fn connect(url: &str) -> io::Result<Self> {
        let inner = web_sys::EventSource::new(url).err_kind(io::ErrorKind::InvalidInput)?;

        // Add an error listener that will store exactly 1 error.
        let (sender, err_receiver) = bounded::<io::Error>(1);
        let _listener = inner.on("error", move |_| {
            let err = io::Error::new(io::ErrorKind::Other, "EventSource connection error");
            let _ = sender.try_send(err);
        });

        // Wait to open.
        let (sender, receiver) = bounded(1);
        let _listener = inner.once("open", move |_| sender.try_send(()).unwrap_throw());
        receiver.recv().await.unwrap_throw();

        let mut this = Self {
            inner,
            err_receiver,
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

    /// Check whether an error has been sent into the channel.
    fn check_err(&mut self) -> io::Result<()> {
        match self.err_receiver.try_recv() {
            Ok(err) => Err(err),
            _ => Ok(()),
        }
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        self.inner.close();
    }
}
