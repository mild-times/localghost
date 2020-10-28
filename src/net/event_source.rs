use super::MessageEvent;
use crate::events::EventListener;
use crate::prelude::*;
use crate::utils::ResultExt;

use async_channel::{self as channel, Receiver, Sender};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, pin::Pin, task::Poll};

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
///
/// # Examples
///
/// ```no_run
/// use localghost::dom::{self, Element, ElementKind};
/// use localghost::prelude::*;
/// use localghost::net::EventSource;
///
/// use futures::stream::StreamExt;
/// use std::io;
///
/// #[localghost::main]
/// async fn main() -> io::Result<()> {
///     // Connect the `EventSource`.
///     let interests = ["fruit"];
///     let mut sse = EventSource::connect("http://localhost:8081/sse", &interests).await?;
///
///     // Create a table
///     let table = Element::new(ElementKind::Table);
///     let tr = Element::new(ElementKind::Tr);
///     tr.append_child(Element::with_text(ElementKind::Th, "name"));
///     tr.append_child(Element::with_text(ElementKind::Th, "data"));
///     table.append_child(tr);
///     dom::body().append_child(&table);
///
///     // For every event in the `EventSource` add an entry to the table.
///     while let Some(ev) = sse.next().await.transpose()? {
///         let tr = Element::new(ElementKind::Tr);
///         tr.append_child(Element::with_text(ElementKind::Td, ev.name()));
///         tr.append_child(Element::with_text(ElementKind::Td, ev.data()));
///         table.append_child(tr);
///     };
///
///     Ok(())
/// }
/// ```
#[pin_project::pin_project(PinnedDrop)]
#[derive(Debug)]
pub struct EventSource {
    /// The internal `EventSource` handle.
    inner: web_sys::EventSource,
    /// The url we connect to.
    url: String,
    /// A listener that catches errors from the stream.
    err_listener: EventListener,
    /// Message sender.
    sender: Sender<MessageEvent>,
    /// Message receiver.
    #[pin]
    receiver: Receiver<MessageEvent>,
    /// Listeners.
    listeners: Vec<EventListener>,
    /// Should we reconnect on error?
    reconnect: Arc<AtomicBool>,
}

impl EventSource {
    /// Create a new instance of `EventSource` and wait for a connection to be
    /// established.
    pub async fn connect<S>(url: &str, interests: &[S]) -> io::Result<Self>
    where
        S: AsRef<str>,
    {
        crate::log::debug!("EventSource({}): connection initiated", url);

        // Initialize the internal url state.
        let url = url.to_owned();
        let inner = web_sys::EventSource::new(&url).err_kind(io::ErrorKind::InvalidInput)?;
        let reconnect = Arc::new(AtomicBool::new(true));

        // Add an error listener that will store exactly 1 error.
        let url2 = url.clone();
        let inner2 = inner.clone();
        let reconnect2 = reconnect.clone();
        let err_listener = inner.on("error", move |_| {
            crate::log::debug!("EventSource({}): remote closed", url2);
            if reconnect2.load(Ordering::SeqCst) == false {
                crate::log::debug!("EventSource({}): instance closed", url2);
                inner2.close();
            } else {
                crate::log::debug!("EventSource({}): instance reconnecting", url2);
            }
        });

        // Wait to open.
        let (sender, receiver) = channel::bounded(1);
        let listener = inner.once("open", move |_| sender.try_send(()).unwrap_throw());
        receiver.recv().await.unwrap_throw();
        drop(listener);

        // Create the instance and check for errors.
        let (sender, receiver) = channel::unbounded();
        let mut this = Self {
            url,
            inner,
            err_listener,
            sender,
            receiver,
            reconnect,
            listeners: vec![],
        };
        for interest in interests {
            let s = interest.as_ref();
            this.register(s);
        }
        this.check_connection()?;

        // All done :~)
        crate::log::debug!("EventSource({}): connection established", this.url);
        Ok(this)
    }

    /// Get whether the instance should reconnect.
    ///
    /// Defaults to `true`.
    pub fn reconnect(&mut self) -> bool {
        self.reconnect.load(Ordering::SeqCst)
    }

    /// Track whether the instance should reconnect.
    pub fn set_reconnect(&mut self, reconnect: bool) {
        self.reconnect.store(reconnect, Ordering::SeqCst);
    }

    /// Register interest in an event.
    pub fn register(&mut self, name: &str) {
        let sender = self.sender.clone();
        let name2 = name.to_owned();
        let listener = EventListener::listen(&self.inner, name.clone(), move |ev| {
            let ev = MessageEvent::from_event(name2.clone(), ev);
            let _ = sender.try_send(ev);
        });
        self.listeners.push(listener);
    }

    /// Receive a message from the stream.
    pub async fn recv(&self) -> io::Result<MessageEvent> {
        let res = self.receiver.recv().await.map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("EventSource({}): receiver error", self.url).as_ref(),
            )
        })?;
        // TODO: race with the error channel here.
        Ok(res)
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

    // Check if connection closed.
    fn check_connection(&self) -> io::Result<()> {
        if !matches!(self.ready_state(), ReadyState::Open) {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                format!("EventSource({})", self.url),
            ))
        } else {
            Ok(())
        }
    }
}

#[pin_project::pinned_drop]
impl PinnedDrop for EventSource {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        this.inner.close();
        crate::log::debug!("EventSource({}): instance closed", this.url);
    }
}

impl AsRef<web_sys::EventTarget> for EventSource {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl async_std::stream::Stream for EventSource {
    type Item = io::Result<MessageEvent>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // Check whether the connection was closed.
        self.check_connection()?;

        // Get the next item from the stream. If no item was found, check
        // whether the connection was closed.
        //
        // We can't use `Self::check_connection` here through the projection.
        let this = self.project();
        let item = match this.receiver.poll_next(cx) {
            Poll::Ready(item) => item,
            Poll::Pending => {
                if this.inner.ready_state() != web_sys::EventSource::OPEN {
                    return Poll::Ready(Some(Err(io::Error::new(
                        io::ErrorKind::NotConnected,
                        format!("EventSource({})", this.url),
                    ))));
                } else {
                    return Poll::Pending;
                }
            }
        };

        // Return the next item.
        Poll::Ready(Ok(item).transpose())
    }
}
