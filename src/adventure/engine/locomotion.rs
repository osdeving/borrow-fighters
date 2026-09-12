//! Draws Rust's independently replaceable walk and kick frames at stable supports.
//!
//! System: Adventure presentation. One cached catalog serves the street,
//! aftermath and chapter paths; gameplay still owns distance and attack timing.

use super::pieces::{PiecePose, StreetPieces};
use crate::{
    adventure::{
        combat::{Action, Actor, Facing},
        locomotion::{Motion, Waking},
    },
    runtime_paths::asset_path,
};
use raylib::prelude::*;
use std::error::Error;

/// Shared body content loaded once per adventure session.
pub struct LocomotionAssets {
    /// Authored poses with local support anchors and attachment sockets.
    pub pieces: StreetPieces,
    /// Editable distance clock and clip names.
    pub motion: Motion,
    /// Existing waking poses and the independently editable room exit route.
    pub waking: Waking,
}

impl LocomotionAssets {
    /// Rejects missing clips before presenting the first scene.
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let pieces =
            StreetPieces::load_catalog(rl, thread, "assets/adventure/locomotion/catalog.json")?;
        let motion = Motion::load(&asset_path("assets/adventure/locomotion/motion.json"))?;
        for id in [&motion.walk_clip, &motion.kick_clip] {
            if !pieces.catalog.pieces.contains_key(id) {
                return Err(format!("missing Rust animation clip {id}").into());
            }
        }
        let waking = Waking::load(&asset_path("assets/adventure/locomotion/waking.json"))?;
        Ok(Self {
            pieces,
            motion,
            waking,
        })
    }

    /// Atomically replaces textures, clip metadata and performance tuning while
    /// the caller retains every gameplay, distance and scene clock.
    pub fn reload(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn Error>> {
        let candidate = Self::load(rl, thread)?;
        *self = candidate;
        Ok(())
    }

    /// Samples locomotion by physical travel and kicks by their authoritative
    /// action age. Anchors refer to the pelvis and sole, never the crop center.
    pub fn draw(&self, d: &mut impl RaylibDraw, actor: &Actor, camera: f32, depth: f32) {
        let id = if actor.action == Action::Kick {
            &self.motion.kick_clip
        } else {
            &self.motion.walk_clip
        };
        let ticks = if actor.action == Action::Kick {
            actor.action_ticks
        } else {
            self.motion.walk_cursor(
                actor.stride_distance,
                1.0,
                self.pieces.catalog.pieces[id].duration(),
            )
        };
        let mut pose = PiecePose::at(Vector2::new(actor.position.x - camera, actor.position.y));
        pose.scale = depth;
        pose.flip = actor.facing == Facing::Left;
        self.pieces.draw(d, id, ticks, &pose);
    }
}

#[cfg(test)]
mod tests {
    use crate::{adventure::scenery::PieceCatalog, runtime_paths::asset_path};

    #[test]
    fn kick_extension_coincides_with_contact_and_recovers_before_unlocking() {
        let catalog =
            PieceCatalog::load(&asset_path("assets/adventure/locomotion/catalog.json")).unwrap();
        let kick = &catalog.pieces["rust.kick"];
        assert!(!kick.looping);
        assert_eq!(kick.duration(), 34);
        for tick in 0..34 {
            let frame = kick.sample(tick).0;
            assert_eq!(
                frame.image.ends_with("pose-09.png"),
                (10..17).contains(&tick)
            );
        }
        let walk = &catalog.pieces["rust.walk"];
        assert_eq!(walk.frames.len(), 8);
        assert!(walk.looping);
        for frame in &walk.frames {
            assert!(asset_path(format!("assets/adventure/{}", frame.image)).is_file());
            assert!(frame.sockets.contains_key("pelvis"));
        }
    }
}
