use js_sys::{Array, Reflect};

use std::fmt::{self, Debug};
use std::iter::{IntoIterator, Iterator};

/// HTTP Headers.
#[derive(Debug)]
pub struct Headers {
    headers: web_sys::Headers,
}

impl Headers {
    /// Create a new instance of `Headers`.
    pub(crate) fn new(headers: web_sys::Headers) -> Self {
        Self { headers }
    }
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
