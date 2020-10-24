use std::fmt::Debug;
use std::io;

use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::net::Headers;
use crate::prelude::*;
use crate::utils::ResultExt;

/// An HTTP Fetch Response.
#[derive(Debug)]
pub struct Response {
    inner: web_sys::Response,
}

impl Response {
    /// Create a new instance of `Response`.
    pub(crate) fn new(res: web_sys::Response) -> Self {
        Self { inner: res }
    }

    /// Access the HTTP headers.
    pub fn headers(&self) -> Headers {
        Headers::new(self.inner.headers())
    }

    /// Get the HTTP return status code.
    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    /// Get the response body as bytes
    ///
    /// # Implementation notes
    ///
    /// This consumes `self` to ensure that the body stream will not throw a
    /// `lock` error.
    ///
    /// # Errors
    ///
    /// An `io::ErrorKind::ConnectionAborted` error will be returned if a
    /// connection error occurred.
    pub async fn body_bytes(self) -> io::Result<Vec<u8>> {
        let promise = self.inner.array_buffer().unwrap_throw();

        // Connections can be "locked" or "disturbed". Because we consume `self`
        // the only error here can be "disturbed" which is akin to an aborted
        // connection.
        let resp = JsFuture::from(promise)
            .await
            .err_kind(io::ErrorKind::ConnectionAborted)?;

        debug_assert!(resp.is_instance_of::<js_sys::ArrayBuffer>());
        let buf: ArrayBuffer = resp.dyn_into().unwrap_throw();
        let slice = Uint8Array::new(&buf);
        let mut buf: Vec<u8> = vec![0; slice.length() as usize];
        slice.copy_to(&mut buf);

        Ok(buf)
    }
}
