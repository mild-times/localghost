use async_std::sync::channel;
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

use coast::task::{self, request_animation_frame, AnimationLoop};

#[wasm_bindgen_test]
async fn animation_loop() {
    let count = Arc::new(Mutex::new(0_u8));

    let mut animator = AnimationLoop::new(|count: Arc<Mutex<u8>>| async move {
        *count.lock().unwrap() += 1;
    });

    animator.render(count.clone()).await;
    assert_eq!(*count.lock().unwrap(), 1);

    animator.render(count.clone()).await;
    assert_eq!(*count.lock().unwrap(), 2);
}

#[wasm_bindgen_test]
async fn raf() {
    let (sender, receiver) = channel(2);

    task::spawn_local(async move {
        let mut count = 0usize;
        loop {
            request_animation_frame().await;
            sender.send(count).await;
            count += 1;
            if count == 2 {
                break;
            }
        }
    });

    let mut results = Vec::new();

    while let Ok(ev) = receiver.recv().await {
        results.push(ev);
    }
    assert_eq!(results, &[0, 1]);
}

#[wasm_bindgen_test]
async fn spawn_local() {
    let handle = coast::task::spawn_local(async move { 12_u8 });
    assert_eq!(handle.await, 12);
}

// #[wasm_bindgen_test]
// async fn spawn_idle() {
//     let handle = coast::task::spawn_idle(|| {
//         12_u8
//     });
//     assert_eq!(handle.await, 12);
// }
