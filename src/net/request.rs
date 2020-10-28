use std::fmt::Debug;
use std::io;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::net::Response;
use crate::prelude::*;
use crate::utils::{self, ResultExt};

/// An HTTP Fetch Request.
#[derive(Debug)]
pub struct Request {
    method: String,
    headers: web_sys::Headers,
    url: String,
}

impl Request {
    /// Create a new `Request`.
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_owned(),
            url: url.to_owned(),
            headers: web_sys::Headers::new().unwrap_throw(),
        }
    }

    /// Create a new `CONNECT` `Request`.
    pub fn connect(url: &str) -> Self {
        Self::new("CONNECT", url)
    }

    /// Create a new `DELETE` `Request`.
    pub fn delete(url: &str) -> Self {
        Self::new("DELETE", url)
    }

    /// Create a new `GET` `Request`.
    pub fn get(url: &str) -> Self {
        Self::new("GET", url)
    }

    /// Create a new `HEAD` `Request`.
    pub fn head(url: &str) -> Self {
        Self::new("HEAD", url)
    }

    /// Create a new `OPTIONS` `Request`.
    pub fn options(url: &str) -> Self {
        Self::new("OPTIONS", url)
    }

    /// Create a new `PATCH` `Request`.
    pub fn patch(url: &str) -> Self {
        Self::new("PATCH", url)
    }

    /// Create a new `POST` `Request`.
    pub fn post(url: &str) -> Self {
        Self::new("POST", url)
    }

    /// Create a new `PUT` `Request`.
    pub fn put(url: &str) -> Self {
        Self::new("PUT", url)
    }

    /// Create a new `TRACE` `Request`.
    pub fn trace(url: &str) -> Self {
        Self::new("TRACE", url)
    }

    /// Get a header.
    pub fn header(&self, name: &str) -> Option<String> {
        self.headers.get(name).unwrap_throw()
    }

    /// Insert a header into the request.
    pub fn insert_header(&self, name: &str, val: &str) {
        self.headers.append(name, val).unwrap_throw();
    }

    /// Check whether the request contains a header.
    pub fn contains_header(&self, name: &str) -> bool {
        self.headers.has(name).unwrap_throw()
    }

    /// Remove a header from the `Request`.
    ///
    /// # Implementation Note
    ///
    /// Unlike other ecosystem crates this does not
    pub fn remove_header(&self, name: &str) {
        self.headers.delete(name).unwrap_throw();
    }

    /// Submit a request
    ///
    /// # Errors
    ///
    /// An error may be returned if the underlying connection returns an error.
    pub async fn send(self) -> Result<Response, io::Error> {
        // Initialize the request config.
        let mut init = web_sys::RequestInit::new();
        init.method(&self.method);
        init.headers(self.headers.as_ref());

        // Send the request.
        let req = web_sys::Request::new_with_str_and_init(&self.url, &init).unwrap_throw();
        let fut = JsFuture::from(utils::window().fetch_with_request(&req));
        let res = fut.await.err_kind(io::ErrorKind::Other)?;
        debug_assert!(res.is_instance_of::<web_sys::Response>());

        Ok(Response::new(res.dyn_into().unwrap_throw()))
    }
}
