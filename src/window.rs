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
/// let window = coast::window();
/// # drop(window)
/// ```
pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("should have a `Window` on the Web")
}

/// Access the browser's `Document` object.
///
/// # Errors
///
/// This function panics if a `Document` is not found.
///
/// # Example
///
/// ```no_run
/// let doc = coast::document();
/// # drop(doc)
/// ```
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect_throw("Could not find `window.document`")
}
