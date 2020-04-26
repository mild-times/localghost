/// Runs a Rust `Future` on the current thread.
///
/// The `future` must be `'static` because it will be scheduled
/// to run in the background and cannot contain any stack references.
///
/// The `future` will always be run on the next microtask tick even if it
/// immediately returns `Poll::Ready`.
///
/// # Panics
///
/// Note that in wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JavaScript exception is thrown. The wasm module is
/// still usable (likely erroneously) after Rust panics.
///
/// If the `future` provided panics then the returned `Promise` **will not
/// resolve**. Instead it will be a leaked promise. This is an unfortunate
/// limitation of wasm currently that's hoped to be fixed one day!
pub use async_std::task::spawn_local;

pub use async_std::task::JoinHandle;

// /// Spawn a task that runs when the event loop is idle.
// #[inline]
// pub fn spawn_idle<F, T>(f: F) -> JoinHandle<T>
// where
//     F: FnOnce() -> T + 'static,
//     T: 'static + Send,
// {
//     let (sender, receiver) = channel();
//     let f2 = Closure::once(Box::new(move || {
//         let t = f();
//         let _ = sender.send(t);
//     }) as Box<dyn FnOnce()>);

//     web_sys::console::time_log();
//     let _ = crate::window().request_idle_callback(f2.as_ref().unchecked_ref());
//     JoinHandle { receiver }
// }
