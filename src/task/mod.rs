//! Types and traits for working with asynchronous tasks.

mod animation_loop;
mod spawn;

pub use animation_loop::AnimationLoop;
pub use spawn::{spawn_local, JoinHandle};
