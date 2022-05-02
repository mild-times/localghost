//! Types and traits for working with asynchronous tasks.

mod animation_frame;
#[cfg(feature = "task-idle")]
mod idle;
mod spawn_local;

pub use animation_frame::AnimationFrame;
#[cfg(feature = "task-idle")]
pub use idle::{Deadline, Idle};
pub use spawn_local::{spawn_local, JoinHandle};
