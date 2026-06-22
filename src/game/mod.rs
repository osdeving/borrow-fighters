//! Contains match-level gameplay state.
//!
//! The greybox prototype keeps world flow small: two fighters, local inputs,
//! damage, match outcome, and restart from the app layer.

pub mod ai;
pub mod arena;
pub mod combat_log;
pub mod feature_flags;
pub mod world;
