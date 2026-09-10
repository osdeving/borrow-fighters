//! Exposes testable Borrow Fighters systems.
//!
//! Runtime code that does not need to live in the binary entrypoint belongs
//! here so gameplay rules can be tested without opening a window.

#[cfg(feature = "adventure")]
pub mod adventure;
#[cfg(feature = "fighting")]
pub mod app;
#[cfg(feature = "fighting")]
pub mod audio;
#[cfg(feature = "fighting")]
pub mod characters;
#[cfg(feature = "fighting")]
pub mod cli;
#[cfg(feature = "fighting")]
pub mod combat;
#[cfg(feature = "fighting")]
pub mod config;
#[cfg(feature = "fighting")]
pub mod engine;
#[cfg(feature = "fighting")]
pub mod game;
#[cfg(feature = "fighting")]
pub mod lore;
pub mod math;
#[cfg(all(feature = "adventure", feature = "fighting"))]
pub mod presentation;
pub mod runtime_paths;
#[cfg(feature = "fighting")]
pub mod scenes;
#[cfg(feature = "fighting")]
pub mod ui;
