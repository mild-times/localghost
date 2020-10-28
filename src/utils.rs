use crate::prelude::*;
use std::io;

/// Convert a `Result<T, JsValue>` to an `io::Result<T>`.
pub(crate) trait ResultExt<T> {
    fn err_kind(self, kind: io::ErrorKind) -> io::Result<T>;
}

impl<T> ResultExt<T> for Result<T, wasm_bindgen::JsValue> {
    fn err_kind(self, kind: io::ErrorKind) -> io::Result<T> {
        self.map_err(|val| {
            let err = js_sys::Error::from(val);
            let msg: String = err.message().into();
            io::Error::new(kind, msg)
        })
    }
}

/// Get a `web_sys::Window`.
pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect_throw("should have a `Window` on the Web")
}

/// Get a `web_sys::Document`.
pub(crate) fn document() -> web_sys::Document {
    window()
        .document()
        .expect_throw("Could not find `window.document`")
}
