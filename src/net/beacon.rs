use crate::prelude::*;

/// Asynchronously send a small amount of data to an HTTP server.
/// 
/// This API is intended to send analytics and diagnostics data to a server
/// before a document is unloaded.
/// 
/// See [MDN -
/// Navigator.sendBeacon](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
/// for more.
/// 
/// # Examples
/// 
/// ```
/// document.on("unload", || {
///     let beacon = Beacon::new();
///     beacon.send_to("localhost:8080/log", b"client closed");
/// });
/// ```
#[derive(Debug)]
pub struct Beacon {
    nav: web_sys::Navigator,
    url: String
}

impl Beacon {
    /// Create a new instance.
    pub fn new(url: String) -> Self {
        Self {
            nav: crate::window().navigator(),
            url,
        }
    }

    /// Send data to the beacon's url.
    pub fn send(&self, data: &mut [u8]) {
        if data.len() > 0 {
            self.nav.send_beacon_with_opt_u8_array(&self.url, Some(data)).unwrap_throw();
        } else {
            self.nav.send_beacon(&self.url).unwrap_throw();
        }
    }
}
