use std::fmt::Debug;
use std::io;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::net::Response;
use crate::prelude::*;
use crate::utils::ResultExt;

/// An HTTP Fetch Request.
#[derive(Debug)]
pub struct Request {
    headers: web_sys::Headers,
    init: web_sys::RequestInit,
    url: String,
}

impl Request {
    /// Create a new instance.
    pub fn new(method: &str, url: &str) -> Self {
        let mut init = web_sys::RequestInit::new();
        init.method(method);
        Self {
            init,
            url: url.to_owned(),
            headers: web_sys::Headers::new().unwrap_throw(),
        }
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
    pub async fn send(mut self) -> Result<Response, io::Error> {
        // Attach the headers to the request data.
        self.init.headers(self.headers.as_ref());

        // Send the request.
        let window = crate::utils::window();
        let request = web_sys::Request::new_with_str_and_init(&self.url, &self.init).unwrap();
        let promise = window.fetch_with_request(&request);
        let resp = JsFuture::from(promise)
            .await
            .err_kind(io::ErrorKind::Other)?;
        debug_assert!(resp.is_instance_of::<web_sys::Response>());
        let res: web_sys::Response = resp.dyn_into().unwrap();

        Ok(Response::new(res))
    }
}
