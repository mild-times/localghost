//! Browser networking APIs

mod beacon;
mod fetch;

pub use beacon::Beacon;
pub use fetch::{Headers, HeadersIter, Request, Response};
