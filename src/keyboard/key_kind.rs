use std::fmt::Display;

/// The computed `key` value of a `KeyboardEvent`
///
/// This `struct` is created by the [`key`] method on [`KeyboardEvent`]. See its
/// documentation for more.
///
/// [`key`]: struct.KeyboardEvent.html#method.key
/// [`KeyboardEvent`]: struct.KeyboardEvent.html
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KeyKind {
    /// The printable representation of a key.
    Key(String),
    /// The value of the key could not be identified.
    Unidentified,
    /// The key is considered a ["dead key"](https://en.wikipedia.org/wiki/Dead_key).
    Dead,
}

impl Display for KeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyKind::Key(s) => write!(f, "{}", s),
            KeyKind::Unidentified => write!(f, "Unidentified"),
            KeyKind::Dead => write!(f, "Dead"),
        }
    }
}
