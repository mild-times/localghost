use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::AddEventListenerOptions;

use crate::events::{Event, EventPhase};

/// A builder used to configure `EventListener`.
#[derive(Debug)]
pub struct Builder {
    passive: bool,
    phase: EventPhase,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            passive: true,
            phase: EventPhase::Bubble,
        }
    }

    /// Set the `EventPhase` during which the event should be triggered.
    ///
    /// Defaults to `EventPhase::Bubble`.
    pub fn phase(mut self, phase: EventPhase) -> Self {
        self.phase = phase;
        self
    }

    /// Set whether the listener should be `passive`.
    ///
    /// Defaults to `true`.
    pub fn passive(mut self, passive: bool) -> Self {
        self.passive = passive;
        self
    }

    /// Register an event listener.
    pub fn listen<T, F>(self, target: T, event_type: &str, f: F) -> EventListener
    where
        T: AsRef<web_sys::EventTarget>,

        F: FnMut(Event) + Clone + 'static,
    {
        let target = target.as_ref();
        let mut f = f.clone();
        let f =
            Closure::wrap(Box::new(move |ev| f(Event::new(ev))) as Box<dyn FnMut(web_sys::Event)>);
        let event_type = event_type.to_owned();

        let mut options = AddEventListenerOptions::new();
        options.once(false);
        options.passive(self.passive);
        options.capture(self.phase.is_capture());

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                f.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        EventListener {
            target: target.clone(),
            event_type,
            f: Some(f),
            phase: self.phase,
        }
    }

    /// Register an event listener that will be called at most once.
    pub fn listen_once<T, F>(target: T, event_type: &str, f: F) -> EventListener
    where
        T: AsRef<web_sys::EventTarget>,

        F: FnOnce(Event) + 'static,
    {
        let target = target.as_ref();
        let f = Closure::once(|ev| f(Event::new(ev)));
        let phase = EventPhase::Bubble;
        let event_type = event_type.to_owned();

        let mut options = AddEventListenerOptions::new();
        options.once(false);
        options.passive(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                f.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        EventListener {
            target: target.clone(),
            event_type,
            f: Some(f),
            phase,
        }
    }
}

/// DOM event listener.
#[derive(Debug)]
#[must_use = "Event listener unsubscribes on drop"]
pub struct EventListener {
    target: web_sys::EventTarget,
    event_type: String,
    f: Option<Closure<dyn FnMut(web_sys::Event)>>,
    phase: EventPhase,
}

impl EventListener {
    /// Register an event listener.
    #[inline]
    pub fn listen<T, F>(target: T, event_type: &str, f: F) -> Self
    where
        T: AsRef<web_sys::EventTarget>,

        F: FnMut(Event) + Clone + 'static,
    {
        Builder::new().listen(target, event_type, f)
    }

    /// Register an event listener that will be called at most once.
    #[inline]
    pub fn listen_once<T, F>(target: T, event_type: &str, f: F) -> Self
    where
        T: AsRef<web_sys::EventTarget>,

        F: FnOnce(Event) + 'static,
    {
        let target = target.as_ref();
        let f = Closure::once(|ev| f(Event::new(ev)));
        let phase = EventPhase::Bubble;
        let event_type = event_type.to_owned();

        let mut options = AddEventListenerOptions::new();
        options.once(false);
        options.passive(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                f.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        Self {
            target: target.clone(),
            event_type,
            f: Some(f),
            phase,
        }
    }

    /// Keeps the `EventListener` alive forever, so it will never be dropped.
    ///
    /// This should only be used when you want the `EventListener` to last
    /// forever, otherwise it will leak memory.
    #[inline]
    pub fn forget(mut self) {
        // take() is necessary because of Rust's restrictions about Drop
        // This will never panic, because `f` is always `Some`
        self.f.take().unwrap_throw().forget()
    }

    /// Returns the `EventTarget`.
    #[inline]
    pub fn target(&self) -> &web_sys::EventTarget {
        &self.target
    }

    /// Returns the event type.
    #[inline]
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Returns whether the event listener is run during the capture or bubble phase.
    ///
    /// The official specification has [a good
    /// explanation](https://www.w3.org/TR/DOM-Level-3-Events/#event-flow) of
    /// capturing vs bubbling.
    #[inline]
    pub fn phase(&self) -> EventPhase {
        self.phase
    }
}

impl Drop for EventListener {
    #[inline]
    fn drop(&mut self) {
        // This will only be None if forget() was called
        if let Some(f) = &self.f {
            self.target
                .remove_event_listener_with_callback_and_bool(
                    self.event_type(),
                    f.as_ref().unchecked_ref(),
                    self.phase.is_capture(),
                )
                .unwrap_throw();
        }
    }
}
