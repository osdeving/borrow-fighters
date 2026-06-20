//! Exposes testable Borrow Fighters systems.
//!
//! Runtime code that does not need to live in the binary entrypoint belongs
//! here so gameplay rules can be tested without opening a window.

pub mod app;
pub mod combat;
pub mod config;
pub mod engine;
pub mod game;
pub mod math;
