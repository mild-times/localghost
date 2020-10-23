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
