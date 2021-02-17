use std::fmt::{self, Debug};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::events::{Event, EventListener};
use crate::prelude::*;

use async_channel::{bounded, Receiver};
use futures_core::{ready, Stream};
use pin_project::pin_project;

/// A type that can register event listeners.
pub trait EventTarget: AsRef<web_sys::EventTarget> {
    /// Register an event listener that may be called multiple times.
    fn on_with<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnMut(Event) + Clone + 'static;

    /// Register an event listener that may be called once.
    ///
    /// After the listener has been called once it removes itself.
    fn once_with<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnOnce(Event) + 'static;

    /// Wait for a stream of events.
    fn on(&self, event_type: &str) -> EventStream {
        let (sender, receiver) = bounded(1);
        let listener = EventListener::listen(self, event_type, move |ev| {
            sender.try_send(ev).unwrap_throw()
        });
        EventStream { listener, receiver }
    }

    /// Wait for a single event.
    fn once(&self, event_type: &str) -> EventFuture {
        let (sender, receiver) = bounded(1);
        let listener = EventListener::listen(self, event_type, move |ev| {
            sender.try_send(ev).unwrap_throw()
        });
        EventFuture {
            listener,
            receiver: Some(receiver),
            fut: None,
        }
    }
}

/// A `Future` returned by `EventTarget::once`.
#[must_use = "Futures do nothing unless awaited"]
pub struct EventFuture {
    listener: EventListener,
    receiver: Option<Receiver<Event>>,
    fut: Option<Pin<Box<dyn Future<Output = Event>>>>,
}

impl Future for EventFuture {
    type Output = Event;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
            let receiver = self.receiver.take().unwrap_throw();
            self.fut = Some(Box::pin(
                async move { receiver.recv().await.unwrap_throw() },
            ));
        }

        let res = ready!(self.fut.as_mut().unwrap_throw().as_mut().poll(cx));
        Poll::Ready(res)
    }
}

impl Debug for EventFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventFuture")
            .field("listener", &self.listener)
            .field("receiver", &self.receiver)
            .field("recv_future", &"Option<dyn Future>")
            .finish()
    }
}

/// A `Future` returned by `EventTarget::on`.
#[pin_project]
#[derive(Debug)]
#[must_use = "Streams do nothing unless awaited"]
pub struct EventStream {
    listener: EventListener,
    #[pin]
    receiver: Receiver<Event>,
}

impl Stream for EventStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.receiver.poll_next(cx)
    }
}

impl<T> EventTarget for T
where
    T: AsRef<web_sys::EventTarget>,
{
    fn on_with<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnMut(Event) + Clone + 'static,
    {
        EventListener::listen(self.as_ref(), event_type, f)
    }

    fn once_with<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnOnce(Event) + 'static,
    {
        EventListener::listen_once(self.as_ref(), event_type, f)
    }
}
