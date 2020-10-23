/// An event emitted by an `EventListener`.
#[derive(Debug)]
pub struct Event {
    inner: web_sys::Event,
}

impl Event {
    /// Create a new instance.
    pub fn new(inner: web_sys::Event) -> Self {
        Self { inner }
    }
}
