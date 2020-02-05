use futures_channel::oneshot::{channel, Sender};

use std::future::Future;
use std::fmt::{self, Debug};
use std::sync::{Mutex, Arc};

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
                    (f.lock().unwrap())(t).await;
                    sender.send(()).unwrap();
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
        let (sender, receiver) = channel();
        (self.f)(sender, value);
        receiver.await.unwrap();
    }
}

impl<T: Send + Sync + 'static> Debug for AnimationLoop<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimationLoop")
            .field("f", &"FnMut(T)")
            .finish()
    }
}
