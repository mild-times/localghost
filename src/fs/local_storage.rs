use crate::prelude::*;
use crate::utils::{window, ResultExt};
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
        match window().local_storage() {
            Ok(Some(storage)) => Ok(Self { storage }),
            _ => Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Unable to access local Storage object.",
            )),
        }
    }

    /// Returns the number of elements in local Storage.
    pub fn len(&self) -> usize {
        self.storage.length().unwrap_throw() as usize
    }

    /// Clears local Storage, removing all key-value pairs.
    pub fn clear(&self) {
        self.storage.clear().unwrap_throw()
    }

    /// Inserts a key-value pair into the local Storage.
    ///
    /// # Errors
    ///
    /// This function may throw an exception if the storage is full.
    pub fn insert(&self, key: &str, val: &str) -> io::Result<()> {
        self.storage
            .set_item(key, val)
            .err_kind(io::ErrorKind::Other)
    }

    /// Removes a key from the local Storage.
    pub fn remove(&self, key: &str) -> Option<String> {
        let val = self.get(key);
        self.storage.remove_item(key).unwrap_throw();
        val
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.get_item(key).ok().flatten()
    }
}
