use std::str::FromStr;

use crate::dom::{Element, ElementKind};
use crate::prelude::*;

/// Return the first element that matches the query.
pub fn query_selector(selectors: &str) -> Option<Element> {
    crate::document()
        .query_selector(selectors)
        .unwrap_throw()
        .map(|el| {
            let kind = ElementKind::from_str(&el.tag_name()).unwrap_throw();
            unsafe { Element::from_raw(kind, el) }
        })
}

// TODO: A `Node` is not an `Element`. We need to cast each `Node` into an
// `Element` using runtime reflection. Maybe something for Ira?
// /// Return the first element that matches the query.
// pub fn query_selector_all(selectors: &str) -> Vec<Element> {
//     let list = crate::document()
//         .query_selector_all(selectors)
//         .unwrap_throw();

//     let mut out = vec![];
//     for i in 0..list.length() {
//         let el = list.get(i).unwrap_throw();
//         let kind = ElementKind::from_str(&el.node_name()).unwrap_throw();
//         out.push(unsafe { Element::from_raw(kind, el) });
//     }
//     out
// }
