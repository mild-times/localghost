use crate::events::EventListener;
use crate::prelude::*;
use crate::utils::ResultExt;

use async_channel::{self as channel, Receiver, Sender};
use std::{io, pin::Pin, task::Poll};
use wasm_bindgen::JsCast;

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
    pub(crate) data: String,
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
    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    /// Convert the message into the data payload.
    pub fn into_bytes(self) -> String {
        self.data
    }
}

/// A receiver of `Server Sent Events` (SSE).
#[pin_project::pin_project(PinnedDrop)]
#[derive(Debug)]
pub struct EventSource {
    /// The internal `EventSource` handle.
    inner: web_sys::EventSource,
    /// The url we connect to.
    url: String,
    /// A listener that catches errors from the stream.
    err_receiver: Receiver<io::Error>,
    /// Message sender.
    sender: Sender<MessageEvent>,
    /// Message receiver.
    #[pin]
    receiver: Receiver<MessageEvent>,
    /// Listeners.
    listeners: Vec<EventListener>,
}

impl EventSource {
    /// Create a new instance of `EventSource` and wait for a connection to be
    /// established.
    pub async fn connect(url: &str) -> io::Result<Self> {
        let url = url.to_owned();
        crate::log::debug!("EventSource({}): connection initiated", url);
        let inner = web_sys::EventSource::new(&url).err_kind(io::ErrorKind::InvalidInput)?;

        // Add an error listener that will store exactly 1 error.
        let url2 = url.clone();
        let (sender, err_receiver) = channel::bounded::<io::Error>(1);
        let inner2 = inner.clone();
        inner.on("error", move |_| {
            crate::log::debug!("EventSource({}): connection error", url2);
            let err = io::Error::new(
                io::ErrorKind::Other,
                format!("EventSource({}) connection error", url2).as_str(),
            );

            crate::log::debug!("EventSource({}): instance closed", url2);
            inner2.close();

            let _ = sender.try_send(err);
        });

        // Wait to open.
        let (sender, receiver) = channel::bounded(1);
        let listener =
            EventListener::listen_once(&inner, "open", move |_| sender.try_send(()).unwrap_throw());
        receiver.recv().await.unwrap_throw();
        drop(listener);

        let (sender, receiver) = channel::unbounded();

        // Create the instance and check for errors.
        let mut this = Self {
            url,
            inner,
            err_receiver,
            sender,
            receiver,
            listeners: vec![],
        };
        this.check_err()?;

        // All done :~)
        crate::log::debug!("EventSource({}): connection established", this.url);
        Ok(this)
    }

    /// Register interest in an event.
    pub fn register(&mut self, name: &'static str) {
        let sender = self.sender.clone();
        let listener = EventListener::listen(&self.inner, name.clone(), move |ev| {
            let ev = ev
                .into_raw()
                .dyn_into::<web_sys::MessageEvent>()
                .unwrap_throw();
            let id = ev.last_event_id();
            let id = match id.len() {
                0 => None,
                _ => Some(id),
            };
            let data: js_sys::JsString = ev.data().into();
            let data: String = data.into();
            let ev = MessageEvent {
                name: name.to_owned(),
                data,
                id,
            };
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

    /// Check whether an error has been sent into the channel.
    fn check_err(&mut self) -> io::Result<()> {
        match self.err_receiver.try_recv() {
            Ok(err) => Err(err),
            _ => Ok(()),
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
        let this = self.project();
        let item = async_std::task::ready!(this.receiver.poll_next(cx));
        // TODO: race with the error stream
        if let Ok(err) = this.err_receiver.try_recv() {
            return Poll::Ready(Some(Err(err)));
        };
        // let item = item.map_err(|_| {
        //     io::Error::new(
        //         io::ErrorKind::Other,
        //         format!("EventSource({}): receiver error", self.url).as_ref(),
        //     )
        // })?;

        Poll::Ready(Ok(item).transpose())
    }
}
