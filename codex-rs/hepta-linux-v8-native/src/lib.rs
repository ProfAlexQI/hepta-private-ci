#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! Linux-only syscall and durable-state primitives for exact v8.
//!
//! This crate is not an admission daemon and exposes no runner signal or
//! production activation entrypoint. Every non-Linux syscall boundary fails
//! closed.

mod durable;
mod error;
mod ipc;
mod recovery;
mod sys;

pub use durable::*;
pub use error::*;
pub use ipc::*;
pub use recovery::*;
pub use sys::*;
