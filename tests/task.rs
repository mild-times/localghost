#![cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;
use std::sync::{Mutex, Arc};

#[wasm_bindgen_test]
async fn animation_loop() {
    let count = Arc::new(Mutex::new(0_u8));
    let mut animator = coast::task::AnimationLoop::new(|count: Arc<Mutex<u8>>| async move {
        *count.lock().unwrap() += 1;
    });

    animator.render(count.clone()).await;
    assert_eq!(*count.lock().unwrap(), 1);

    animator.render(count.clone()).await;
    assert_eq!(*count.lock().unwrap(), 2);
}
