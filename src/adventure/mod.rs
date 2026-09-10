//! Owns the isolated Rust story adventure, including its scenes and combat.
//!
//! System: Adventure. Only neutral math and runtime paths cross into this domain;
//! no fighting-game scene, actor, tuning or presentation is reused here.

pub mod ambient;
pub mod app;
pub mod arrival;
pub mod combat;
pub mod engine;
pub mod neighborhood;
pub mod scenery;
pub mod story;
pub mod text;
