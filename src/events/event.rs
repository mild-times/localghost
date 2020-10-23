use wasm_bindgen::JsCast;

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

    /// Attempt to acquire the target element by casting into it.
    pub fn target<T>(&self) -> Option<T>
    where
        T: wasm_bindgen::JsCast,
    {
        self.inner.target().and_then(|t| t.dyn_into::<T>().ok())
    }

    pub fn into_raw(self) -> web_sys::Event {
        self.inner
    }
}

impl AsRef<wasm_bindgen::JsValue> for Event {
    fn as_ref(&self) -> &wasm_bindgen::JsValue {
        self.inner.as_ref()
    }
}

impl Into<wasm_bindgen::JsValue> for Event {
    fn into(self) -> wasm_bindgen::JsValue {
        self.inner.into()
    }
}

impl AsRef<web_sys::Event> for Event {
    fn as_ref(&self) -> &web_sys::Event {
        &self.inner
    }
}
