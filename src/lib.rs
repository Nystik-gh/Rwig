#![feature(unwrap_infallible)]

mod capture;
mod hook;
mod ipc;
mod stream;

mod common;

pub use common::audio;

pub use capture::Capture;
pub use hook::{CompatibilityInfo, Injector};
pub use stream::CaptureStream;
