/// A `fetch` body.
#[derive(Debug)]
pub struct Body {
    body: web_sys::ReadableStream,
    next: Option<()>,
}
