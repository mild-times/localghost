use crate::events::{Event, EventListener};

/// A type that can register event listeners.
pub trait EventTarget {
    /// Register an event listener.
    fn on<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnMut(Event) + Clone + 'static;

    /// Register an event listener that will be called at most once.
    fn once<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnOnce(Event) + 'static;
}

impl<T> EventTarget for T
where
    T: AsRef<web_sys::EventTarget>,
{
    fn on<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnMut(Event) + Clone + 'static,
    {
        EventListener::listen(self.as_ref(), event_type, f)
    }

    fn once<F>(&self, event_type: &str, f: F) -> EventListener
    where
        F: FnOnce(Event) + 'static,
    {
        EventListener::listen_once(self.as_ref(), event_type, f)
    }
}
