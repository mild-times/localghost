#![cfg(target_arch = "wasm32")]

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use futures_channel::oneshot;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn promise_resolve_is_ok_future() {
    let p = js_sys::Promise::resolve(&JsValue::from(42));
    let x = JsFuture::from(p).await.unwrap();
    assert_eq!(x, 42);
}
