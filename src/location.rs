use crate::prelude::*;
use crate::utils::document;
use crate::utils::ResultExt;

use std::{fmt, io};

/// Representation of the location (URL) of the object it is linked to.
#[derive(Debug)]
pub struct Location {
    inner: web_sys::Location,
}

impl Location {
    /// Create a new instance of `Location` by reading `document.location`.
    pub fn new() -> Self {
        Self {
            inner: document().location().unwrap_throw(),
        }
    }

    /// Access the URL at the current location.
    pub fn href(&self) -> String {
        self.inner.href().unwrap_throw()
    }

    /// Loads the resource at the URL provided in parameter.
    ///
    /// # Errors
    ///
    /// An error may be returned if the url is malformed.
    pub fn assign(&self, url: &str) -> io::Result<()> {
        self.inner
            .assign(url)
            .err_kind(io::ErrorKind::InvalidInput)?;
        Ok(())
    }

    /// Replaces the current resource with the one at the provided URL.
    ///
    /// The difference from the `assign` method is that after using `replace` the
    /// current page will not be saved in session `History`, meaning the user
    /// won't be able to use the back button to navigate to it
    ///
    /// # Errors
    ///
    /// An error may be returned if the url is malformed.
    pub fn replace(&self, url: &str) -> io::Result<()> {
        self.inner
            .replace(url)
            .err_kind(io::ErrorKind::InvalidInput)?;
        Ok(())
    }

    /// Reloads the current URL, like the Refresh button.
    ///
    /// # Errors
    ///
    /// An error may be returned if the origin of the script calling `reload`
    /// differs from the origin of the page that owns the `Location` object.
    pub fn reload(&self) -> io::Result<()> {
        self.inner
            .reload()
            .err_kind(io::ErrorKind::PermissionDenied)?;
        Ok(())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.href())
    }
}
