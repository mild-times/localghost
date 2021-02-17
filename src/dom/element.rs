use std::borrow::Cow;

use crate::dom::Text;
use crate::prelude::*;

/// An HTML element.
#[derive(Debug)]
pub struct Element {
    kind: Cow<'static, str>,
    el: web_sys::Element,
}

impl Element {
    /// Create a new instance.
    pub fn new<S>(kind: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let kind = kind.into();
        let el = crate::utils::document()
            .create_element(&kind)
            .unwrap_throw();
        Self { kind, el }
    }

    /// Create a new instance with an internal text node.
    pub fn with_text<S>(kind: S, text: &str) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let kind = kind.into();
        let el = crate::utils::document()
            .create_element(&kind)
            .unwrap_throw();
        let this = Self { kind, el };
        this.append(Text::new(text));
        this
    }

    /// Create a new instance from a `web_sys::Element` and an `&'static str`.
    pub unsafe fn from_raw<S>(kind: S, el: web_sys::Element) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let kind = kind.into();
        Self { el, kind }
    }

    /// Append a child element.
    pub fn append<C>(&self, child: C)
    where
        C: AsRef<web_sys::Node>,
    {
        self.el.append_child(child.as_ref()).unwrap_throw();
    }

    /// Gets the value of an attribute on the specified element.    pub fn set_attr(&self, name: &str, value: &str) {
    pub fn attr(&self, name: &str) -> Option<String> {
        self.el.get_attribute(name)
    }

    /// Sets the value of an attribute on the specified element. If the attribute
    /// already exists, the value is updated; otherwise a new attribute is added
    /// with the specified name and value.
    pub fn set_attr(&self, name: &str, value: &str) {
        self.el.set_attribute(name, value).unwrap_throw();
    }

    /// Return the first element that matches the query.
    pub fn query_selector(&self, selectors: &str) -> Option<Element> {
        self.el
            .query_selector(selectors)
            .unwrap_throw()
            .map(|el| unsafe { Element::from_raw(el.tag_name(), el) })
    }

    /// Get the `textContent` field of this object.
    pub fn text(&self) -> Option<String> {
        self.el.text_content()
    }

    /// Set the `textContent` field of this object.
    pub fn set_text(&self, value: &str) {
        self.el.set_text_content(Some(value));
    }

    /// Clear the `textContent` field of this object.
    pub fn clear_text(&self) {
        self.el.set_text_content(None);
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
