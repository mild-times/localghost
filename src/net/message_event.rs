use crate::events::Event;
use crate::prelude::*;
use wasm_bindgen::JsCast;

/// An SSE event with a data payload.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MessageEvent {
    /// The ID of this event.
    pub(crate) id: Option<String>,
    /// The event name.
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
    pub fn into_string(self) -> String {
        self.data
    }

    /// Create a new instance from a name and an `Event`.
    pub fn from_event(name: String, ev: Event) -> Self {
        let ev = ev
            .into_raw()
            .dyn_into::<web_sys::MessageEvent>()
            .unwrap_throw();
        Self::from_raw(name, ev)
    }

    /// Create a new instance from a raw `MessageEvent` and a name.
    pub fn from_raw(name: String, ev: web_sys::MessageEvent) -> Self {
        let id = ev.last_event_id();
        let id = match id.len() {
            0 => None,
            _ => Some(id),
        };
        let data: js_sys::JsString = ev.data().into();
        let data: String = data.into();
        Self {
            name: name.to_owned(),
            data,
            id,
        }
    }
}
