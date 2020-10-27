use crate::prelude::*;
use crate::utils;
use std::io;

/// Access a persistent storage object for the Document's origin.
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
#[derive(Debug)]
pub struct LocalStorage {
    storage: web_sys::Storage,
}

impl LocalStorage {
    /// Access the local Storage object for a Document's origin.
    ///
    /// # Errors
    ///
    /// This function will return an error if a request violates a policy decision, or the origin
    /// is not a valid scheme/host/port tuple.
    pub fn open() -> io::Result<Self> {
        let window = utils::window();
        match window.local_storage() {
            Ok(Some(storage)) => Ok(Self { storage }),
            _ => Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Unable to access local Storage object.",
            )),
        }
    }
}
