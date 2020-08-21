use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// Access the browser's `Window` object.
///
/// # Errors
///
/// This function panics if a `Window` is not found.
///
/// # Example
///
/// ```no_run
/// let window = localghost::window();
/// # drop(window)
/// ```
pub fn window() -> Window {
    Window::new()
}

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

impl Deref for Window {
    type Target = web_sys::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
