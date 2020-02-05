//! Event listeners.

use std::borrow::Cow;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{AddEventListenerOptions, Event};

mod event_target;

pub use event_target::EventTarget;

/// Specifies the phase during which the event listener is run.
///
/// See [W3C UI Events: event
/// flow](https://www.w3.org/TR/DOM-Level-3-Events/#event-flow) for more details.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EventPhase {
    /// The event object propagates through the target’s ancestors in reverse
    /// order, starting with the target’s parent and ending with the Window. This
    /// phase is also known as the bubbling phase.
    Bubble,
    /// The event object propagates through the target’s ancestors from the
    /// Window to the target’s parent. This phase is also known as the capturing
    /// phase.
    Capture,
}

impl EventPhase {
    /// Returns `true` if the phase is `Capture`.
    #[inline]
    pub fn is_capture(&self) -> bool {
        self == &EventPhase::Capture
    }

    /// Returns `true` if the phase is `Bubble`.
    #[inline]
    pub fn is_bubble(&self) -> bool {
        self == &EventPhase::Bubble
    }
}

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
    pub fn listen<S, F>(self, target: &web_sys::EventTarget, event_type: S, f: F) -> EventListener
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        let f = Closure::wrap(Box::new(f) as Box<dyn FnMut(&Event)>);
        let event_type = event_type.into();

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
    pub fn listen_once<S, F>(target: &web_sys::EventTarget, event_type: S, f: F) -> EventListener
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        let f = Closure::once(f);
        let phase = EventPhase::Bubble;
        let event_type = event_type.into();

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
    event_type: Cow<'static, str>,
    f: Option<Closure<dyn FnMut(&Event)>>,
    phase: EventPhase,
}

impl EventListener {
    /// Register an event listener.
    #[inline]
    pub fn listen<S, F>(target: &web_sys::EventTarget, event_type: S, f: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        Builder::new().listen(target, event_type, f)
    }

    /// Register an event listener that will be called at most once.
    #[inline]
    pub fn listen_once<S, F>(target: &web_sys::EventTarget, event_type: S, f: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        let f = Closure::once(f);
        let phase = EventPhase::Bubble;
        let event_type = event_type.into();

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
