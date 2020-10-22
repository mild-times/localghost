use crate::events::EventListener;
use std::borrow::Cow;
use web_sys::Event;

/// A type that can register event listeners.
pub trait EventTarget {
    /// Register an event listener.
    fn on<S, F>(&self, event_type: S, f: F)
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static;

    /// Register an event listener that will be called at most once.
    fn once<S, F>(&self, event_type: S, f: F)
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static;
}

impl<T> EventTarget for T
where
    T: AsRef<web_sys::EventTarget>,
{
    fn on<S, F>(&self, event_type: S, f: F)
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        EventListener::listen(self.as_ref(), event_type, f).forget();
    }

    fn once<S, F>(&self, event_type: S, f: F)
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        EventListener::listen_once(self.as_ref(), event_type, f).forget();
    }
}
