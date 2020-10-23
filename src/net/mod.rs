//! Browser networking APIs

mod beacon;
mod fetch;
mod sse;

pub use beacon::Beacon;
pub use fetch::{Headers, HeadersIter, Request, Response};
pub use sse::{EventSource, ReadyState};
