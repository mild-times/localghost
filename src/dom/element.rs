use crate::dom::ElementKind;
use crate::prelude::*;

/// An HTML element.
pub struct Element {
    kind: ElementKind,
    el: web_sys::HtmlElement,
}

impl Element {
    pub fn new(kind: ElementKind) -> Self {
        let el = crate::document()
            .create_element(kind.as_str())
            .unwrap_throw();
        Self { kind, el }
    }

    pub unsafe fn from_raw(kind: ElementKind, el: web_sys::HtmlElement) -> Self {
        Self { el, kind }
    }
}

impl AsRef<web_sys::Node> for Element {
    fn as_ref(&self) -> &web_sys::Node {
        self.el.as_ref()
    }
}
