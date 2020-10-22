use std::fmt::{self, Debug};
use std::io;

use js_sys::{Array, ArrayBuffer, Reflect, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

use std::iter::{IntoIterator, Iterator};

/// An HTTP Fetch Request.
#[derive(Debug)]
pub struct Request {
    init: RequestInit,
    url: String,
}

impl Request {
    /// Create a new instance.
    pub fn new(method: impl AsRef<str>, url: impl AsRef<str>) -> Self {
        let mut init = web_sys::RequestInit::new();
        init.method(method.as_ref());
        Self {
            init,
            url: url.as_ref().to_owned(),
        }
    }

    /// Submit a request
    // TODO(yoshuawuyts): turn this into a `Future` impl on `Request` instead.
    pub async fn send(self) -> Result<Response, io::Error> {
        // Send the request.
        let window = crate::window();
        let request = web_sys::Request::new_with_str_and_init(&self.url, &self.init).unwrap();
        let promise = window.fetch_with_request(&request);
        let resp = JsFuture::from(promise).await.unwrap();
        debug_assert!(resp.is_instance_of::<web_sys::Response>());
        let res: web_sys::Response = resp.dyn_into().unwrap();

        // Get the response body.
        let promise = res.array_buffer().unwrap();
        let resp = JsFuture::from(promise).await.unwrap();
        debug_assert!(resp.is_instance_of::<js_sys::ArrayBuffer>());
        let buf: ArrayBuffer = resp.dyn_into().unwrap();
        let slice = Uint8Array::new(&buf);
        let mut body: Vec<u8> = vec![0; slice.length() as usize];
        slice.copy_to(&mut body);

        Ok(Response::new(res, body))
    }
}

/// An HTTP Fetch Response.
#[derive(Debug)]
pub struct Response {
    res: web_sys::Response,
    body: Option<Vec<u8>>,
}

impl Response {
    fn new(res: web_sys::Response, body: Vec<u8>) -> Self {
        Self {
            res,
            body: Some(body),
        }
    }

    /// Access the HTTP headers.
    pub fn headers(&self) -> Headers {
        Headers {
            headers: self.res.headers(),
        }
    }

    /// Get the request body as a byte vector.
    ///
    /// Returns an empty vector if the body has already been consumed.
    pub fn body_bytes(&mut self) -> Vec<u8> {
        self.body.take().unwrap_or_else(|| vec![])
    }

    /// Get the HTTP return status code.
    pub fn status(&self) -> u16 {
        self.res.status()
    }
}

/// HTTP Headers.
#[derive(Debug)]
pub struct Headers {
    headers: web_sys::Headers,
}

impl IntoIterator for Headers {
    type Item = (String, String);
    type IntoIter = HeadersIter;

    fn into_iter(self) -> Self::IntoIter {
        HeadersIter {
            iter: js_sys::try_iter(&self.headers).unwrap().unwrap(),
        }
    }
}

/// HTTP Headers Iterator.
pub struct HeadersIter {
    iter: js_sys::IntoIter,
}

impl Debug for HeadersIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeadersIter").finish()
    }
}

impl Iterator for HeadersIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.iter.next()?;

        let array: Array = pair.unwrap().into();
        let vals = array.values();

        let prop = String::from("value").into();
        let key = Reflect::get(&vals.next().unwrap(), &prop).unwrap();
        let value = Reflect::get(&vals.next().unwrap(), &prop).unwrap();

        Some((
            key.as_string().to_owned().unwrap(),
            value.as_string().to_owned().unwrap(),
        ))
    }
}
