use std::fmt::Debug;
use std::io;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

use crate::net::Response;

/// An HTTP Fetch Request.
#[derive(Debug)]
pub struct Request {
    init: RequestInit,
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
        }
    }

    /// Submit a request
    pub async fn send(self) -> Result<Response, io::Error> {
        // Send the request.
        let window = crate::utils::window();
        let request = web_sys::Request::new_with_str_and_init(&self.url, &self.init).unwrap();
        let promise = window.fetch_with_request(&request);
        let resp = JsFuture::from(promise).await.unwrap();
        debug_assert!(resp.is_instance_of::<web_sys::Response>());
        let res: web_sys::Response = resp.dyn_into().unwrap();

        // Get the response body.
        // let promise = res.array_buffer().unwrap();
        // let resp = JsFuture::from(promise).await.unwrap();
        // debug_assert!(resp.is_instance_of::<js_sys::ArrayBuffer>());
        // let buf: ArrayBuffer = resp.dyn_into().unwrap();
        // let slice = Uint8Array::new(&buf);
        // let mut body: Vec<u8> = vec![0; slice.length() as usize];
        // slice.copy_to(&mut body);

        Ok(Response::new(res))
    }
}
