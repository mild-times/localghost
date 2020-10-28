/// An HTML Text node
#[derive(Debug)]
pub struct Text {
    inner: web_sys::Text,
}

impl Text {
    /// Create a new instance.
    pub fn new(text: &str) -> Self {
        let inner = crate::utils::document().create_text_node(text);
        Self { inner }
    }
}

impl AsRef<web_sys::Node> for Text {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}
