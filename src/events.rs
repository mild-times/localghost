//! Event listeners.

use std::borrow::Cow;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{AddEventListenerOptions, Event, EventTarget};

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

/// DOM event listener.
#[derive(Debug)]
#[must_use = "Event listener unsubscribes on drop"]
pub struct EventListener {
    target: EventTarget,
    event_type: Cow<'static, str>,
    callback: Option<Closure<dyn FnMut(&Event)>>,
    phase: EventPhase,
}

impl EventListener {
    /// Registers an event listener on an
    /// [`EventTarget`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html).
    #[inline]
    pub fn new<S, F>(target: &EventTarget, event_type: S, callback: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(&Event)>);
        let phase = EventPhase::Bubble;
        let event_type = event_type.into();

        let mut options = AddEventListenerOptions::new();
        options.once(false);
        options.passive(true);
        options.capture(false);

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                callback.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
            phase,
        }
    }

    /// This is exactly the same as [`EventListener::new`](#method.new), except the event will only fire once,
    /// and it accepts `FnOnce` instead of `FnMut`.
    #[inline]
    pub fn once<S, F>(target: &EventTarget, event_type: S, callback: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        let callback = Closure::once(callback);
        let phase = EventPhase::Bubble;
        let event_type = event_type.into();

        let mut options = AddEventListenerOptions::new();
        options.once(false);
        options.passive(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                callback.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
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
        // This will never panic, because `callback` is always `Some`
        self.callback.take().unwrap_throw().forget()
    }

    /// Returns the `EventTarget`.
    #[inline]
    pub fn target(&self) -> &EventTarget {
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
        if let Some(callback) = &self.callback {
            self.target
                .remove_event_listener_with_callback_and_bool(
                    self.event_type(),
                    callback.as_ref().unchecked_ref(),
                    self.phase.is_capture(),
                )
                .unwrap_throw();
        }
    }
}
