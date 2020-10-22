use crate::dom::ElementKind;
use crate::prelude::*;

/// An HTML element.
#[derive(Debug)]
pub struct Element {
    kind: ElementKind,
    el: web_sys::Element,
}

impl Element {
    /// Create a new instance.
    pub fn new(kind: ElementKind) -> Self {
        let el = crate::document()
            .create_element(kind.as_str())
            .unwrap_throw();
        Self { kind, el }
    }

    /// Create a new instance from a `web_sys::Element` and an `ElementKind`.
    pub unsafe fn from_raw(kind: ElementKind, el: web_sys::Element) -> Self {
        Self { el, kind }
    }

    /// Append a child element.
    pub fn append_child<C>(&mut self, child: C)
    where
        C: AsRef<web_sys::Node>,
    {
        self.el.append_child(child.as_ref()).unwrap_throw();
    }
}

impl AsRef<web_sys::Node> for Element {
    fn as_ref(&self) -> &web_sys::Node {
        self.el.as_ref()
    }
}

impl AsRef<web_sys::EventTarget> for Element {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.el.as_ref()
    }
}
