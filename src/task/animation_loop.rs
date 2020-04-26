use std::fmt::{self, Debug};
use std::future::Future;

use async_std::sync::{channel, Arc, Mutex, Receiver, Sender};

use crate::task::spawn_local;

/// An animation frame loop.
///
/// # Example
///
/// ```no_run
/// let mut animator = AnimationLoop::new(|s| async move {
///     s.send(()).await;
/// });
///
/// while let Some(_) in receiver.next().await {
///     animator.render(sender.clone()).await;
/// }
/// ```
pub struct AnimationLoop<T: Send + Sync + 'static> {
    f: Box<dyn Fn(Sender<()>, T)>,
}

impl<T: Send + Sync + 'static> AnimationLoop<T> {
    /// Create a new `RequestAnimationFrame` loop.
    pub fn new<F, Fut>(f: F) -> Self
    where
        Fut: Future<Output = ()> + 'static,
        F: Fn(T) -> Fut + 'static,
    {
        let f = Arc::new(Mutex::new(f));

        Self {
            f: Box::new(move |sender, t| {
                let f = f.clone();
                spawn_local(async move {
                    (f.lock().await)(t).await;
                    sender.send(()).await
                });
            }),
        }
    }

    /// Schedule the the animation loop to render on the next tick of the event
    /// loop.
    ///
    /// This function will wait until the frame has finished rendering. This can
    /// be run in a loop to render frames as they come in.
    pub async fn render(&mut self, value: T) {
        let (sender, receiver) = channel(1);
        (self.f)(sender, value);
        receiver.recv().await;
    }
}

impl<T: Send + Sync + 'static> Debug for AnimationLoop<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimationLoop")
            .field("f", &"FnMut(T)")
            .finish()
    }
}

use async_std::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

#[derive(Debug)]
pub struct AnimationFrameHandle {
    animation_id: i32,
    receiver: Receiver<u64>,
    closure: Closure<dyn FnMut(JsValue)>,
}

impl Future for AnimationFrameHandle {
    type Output = u64;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.receiver).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(val)) => Poll::Ready(val),
            Poll::Ready(None) => panic!("receiver dropped too early"),
        }
    }
}

impl Drop for AnimationFrameHandle {
    fn drop(&mut self) {
        web_sys::window()
            .expect("must have window")
            .cancel_animation_frame(self.animation_id);
    }
}

pub fn request_animation_frame() -> AnimationFrameHandle {
    let (sender, receiver) = channel(1);

    let cb = Closure::once(Box::new(move |val: JsValue| {
        let val = val.as_f64().expect("invalid argument") as u64;
        spawn_local(async move {
            sender.send(val).await;
        });
    }) as Box<dyn FnOnce(JsValue)>);

    let animation_id = web_sys::window()
        .expect("must have window")
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap();

    AnimationFrameHandle {
        animation_id,
        receiver,
        closure: cb,
    }
}
