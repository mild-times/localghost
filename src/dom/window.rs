use crate::prelude::*;

/// A reference to the `Window` object.
#[derive(Debug)]
pub struct Window {
    window: web_sys::Window,
}

impl Window {
    /// Create a new instance of `Window`.
    pub fn new() -> Self {
        let window = web_sys::window().expect_throw("should have a `Window` on the Web");
        Self { window }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}
