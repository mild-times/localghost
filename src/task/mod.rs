//! Types and Traits for working with asynchronous tasks.

mod animation_loop;
mod spawn;

pub use animation_loop::{request_animation_frame, AnimationFrameHandle, AnimationLoop};
pub use spawn::{spawn_local, JoinHandle};
