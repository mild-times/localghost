use std::fmt::Debug;
use std::io;

use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::net::{Headers, HeadersIter};
use crate::prelude::*;
use crate::utils::ResultExt;

/// An HTTP Fetch Response.
#[derive(Debug)]
pub struct Response {
    inner: web_sys::Response,
    headers: Headers,
}

impl Response {
    /// Create a new instance of `Response`.
    pub(crate) fn new(res: web_sys::Response) -> Self {
        let headers = Headers::new(res.headers());
        let inner = res;
        Self { inner, headers }
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
        let fut = JsFuture::from(self.inner.array_buffer().unwrap_throw());

        // Connections can be "locked" or "disturbed". Because we consume `self`
        // the only error here can be "disturbed" which is akin to an aborted
        // connection.
        let res = fut.await.err_kind(io::ErrorKind::ConnectionAborted)?;

        debug_assert!(res.is_instance_of::<js_sys::ArrayBuffer>());
        let buf: ArrayBuffer = res.dyn_into().unwrap_throw();
        let slice = Uint8Array::new(&buf);
        let mut buf: Vec<u8> = vec![0; slice.length() as usize];
        slice.copy_to(&mut buf);

        Ok(buf)
    }

    /// Get the response body as a String.
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
    pub async fn body_string(self) -> io::Result<String> {
        let fut = JsFuture::from(self.inner.text().unwrap_throw());

        // Connections can be "locked" or "disturbed". Because we consume `self`
        // the only error here can be "disturbed" which is akin to an aborted
        // connection.
        let res = fut.await.err_kind(io::ErrorKind::ConnectionAborted)?;

        debug_assert!(res.is_instance_of::<js_sys::JsString>());
        let string: js_sys::JsString = res.dyn_into().unwrap_throw();

        Ok(string.into())
    }

    /// Get a header.
    pub fn header(&self, name: &str) -> Option<String> {
        self.headers.inner.get(name).unwrap_throw()
    }

    /// Insert a header into the request.
    pub fn insert_header(&self, name: &str, val: &str) {
        self.headers.inner.append(name, val).unwrap_throw();
    }

    /// Check whether the request contains a header.
    pub fn contains_header(&self, name: &str) -> bool {
        self.headers.inner.has(name).unwrap_throw()
    }

    /// Remove a header from the `Request`.
    ///
    /// # Implementation Note
    ///
    /// Unlike other ecosystem crates this does not
    pub fn remove_header(&self, name: &str) {
        self.headers.inner.delete(name).unwrap_throw();
    }

    /// Get an iterator over all headers.
    pub fn headers(&self) -> HeadersIter {
        self.headers.iter()
    }

    /// Get the length of the body if it's been set.
    pub fn body_len(&self) -> Option<usize> {
        self.header("content-length")
            .map(|s| s.parse().ok())
            .flatten()
    }
}
