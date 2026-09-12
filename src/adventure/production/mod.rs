//! Shares validated character combat and deterministic simulation with production tools.
//!
//! System: Adventure production. Chapter hosts and the external lab consume this
//! domain; rendering, file I/O and the separate fighting game remain outside it.

pub mod animation;
pub mod sim;
pub mod spec;

pub use sim::{
    Action, Actor, ActorId, Bounds, EnemySpawn, Event, Facing, Input, Outcome, Projectile,
    ProjectileId, Simulation, Team,
};
pub use spec::{
    AiSpec, CharacterPack, CharacterSpec, CombatCatalog, GuardSpec, HitboxWindow, Loadout,
    MoveLibrary, MoveSpec, MovementSpec, ProjectileSpec, TICKS_PER_SECOND,
};

#[cfg(test)]
mod tests;
