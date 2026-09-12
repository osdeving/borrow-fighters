//! Draws Rust's interpolated run rig and authored walk/kick frames at stable supports.
//!
//! System: Adventure presentation. One cached catalog serves the street,
//! aftermath and chapter paths; gameplay still owns distance and attack timing.

use super::pieces::{PiecePose, StreetPieces, TexturePoint};
use crate::{
    adventure::{
        combat::{Action, Actor, Facing},
        locomotion::{
            Gait, Motion, TravelView, Waking,
            mesh::RunMesh,
            run::{RunRig, rotate},
        },
    },
    math::vec2::Vec2,
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
    /// Interpolated anatomical run with boot contacts read from source sockets.
    pub run: RunRig,
    /// Continuous cloth, joint weights and protected body bands.
    pub mesh: RunMesh,
    soles: [Vec2; 2],
}

impl LocomotionAssets {
    /// Rejects missing clips before presenting the first scene.
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let pieces =
            StreetPieces::load_catalog(rl, thread, "assets/adventure/locomotion/catalog.json")?;
        let motion = Motion::load(&asset_path("assets/adventure/locomotion/motion.json"))?;
        for id in [
            &motion.walk_clip,
            &motion.kick_clip,
            &motion.back_walk_clip,
            &motion.front_walk_clip,
        ] {
            if !pieces.catalog.pieces.contains_key(id) {
                return Err(format!("missing Rust animation clip {id}").into());
            }
        }
        let waking = Waking::load(&asset_path("assets/adventure/locomotion/waking.json"))?;
        let run = RunRig::load(&asset_path("assets/adventure/locomotion/run-rig.json"))?;
        let mesh = RunMesh::load(&asset_path("assets/adventure/locomotion/run-mesh.json"))?;
        if run.clip != motion.run_clip {
            return Err("run clip id differs between motion and rig".into());
        }
        for id in run.pieces() {
            if !pieces.catalog.pieces.contains_key(id) {
                return Err(format!("missing running piece {id}").into());
            }
        }
        let mut soles = [Vec2::ZERO; 2];
        for (i, leg) in [&run.near, &run.far].into_iter().enumerate() {
            let sole = pieces
                .socket(&leg.boot, 0, &PiecePose::at(Vector2::zero()), "sole")
                .ok_or("running boot requires sole socket")?;
            soles[i] = Vec2::new(sole.x, sole.y);
        }
        let leg_frame = pieces
            .catalog
            .pieces
            .get(&mesh.leg_piece)
            .ok_or("missing continuous running leg")?
            .sample(0)
            .0;
        let body_height = pieces.catalog.pieces[&run.body_piece].sample(0).0.source[3];
        mesh.validate_crops([leg_frame.source[2], leg_frame.source[3]], body_height)?;
        run.validate_performance(motion.run_stride_pixels, soles)?;
        for name in ["near_shoulder", "far_shoulder"] {
            if pieces
                .socket(&run.body_piece, 0, &PiecePose::at(Vector2::zero()), name)
                .is_none()
            {
                return Err(format!("running body requires {name} socket").into());
            }
        }
        Ok(Self {
            pieces,
            motion,
            waking,
            run,
            mesh,
            soles,
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
        if actor.action == Action::Walk && actor.gait == Gait::Run {
            self.draw_run(d, actor, camera, depth);
            return;
        }
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

    /// Keeps the anatomical orientation aligned with travel through depth.
    pub fn draw_crossing(
        &self,
        d: &mut impl RaylibDraw,
        actor: &Actor,
        view: TravelView,
        depth: f32,
    ) {
        let id = match view {
            TravelView::Back => &self.motion.back_walk_clip,
            TravelView::Front => &self.motion.front_walk_clip,
            TravelView::Side => {
                self.draw(d, actor, 0.0, depth);
                return;
            }
        };
        let ticks = self.motion.walk_cursor(
            actor.stride_distance,
            1.0,
            self.pieces.catalog.pieces[id].duration(),
        );
        let mut pose = PiecePose::at(Vector2::new(actor.position.x, actor.position.y));
        pose.scale = depth;
        self.pieces.draw(d, id, ticks, &pose);
    }

    fn draw_run(&self, d: &mut impl RaylibDraw, actor: &Actor, camera: f32, depth: f32) {
        let sampled = self.run.sample(
            self.motion.phase_for(Gait::Run, actor.stride_distance),
            self.motion.run_stride_pixels,
            self.soles,
        );
        let origin = Vector2::new(actor.position.x - camera, actor.position.y);
        let sign = actor.facing.sign();
        let at = |point: Vec2, rotation: f32| PiecePose {
            position: Vector2::new(
                origin.x + point.x * depth * sign,
                origin.y + point.y * depth,
            ),
            scale: depth,
            flip: sign < 0.0,
            rotation: rotation * sign,
            tint: Color::WHITE,
        };
        let (body_frame, body_scale) = self.pieces.catalog.pieces[&self.run.body_piece].sample(0);
        let body_point = |x: f32, y: f32| {
            let mapped = Vec2::new(
                (x - body_frame.anchor[0]) * body_scale,
                (self.mesh.body_y(y) - self.mesh.body_y(body_frame.anchor[1])) * body_scale,
            );
            let rotated = rotate(mapped, sampled.lean);
            Vec2::new(sampled.hip.x + rotated.x, sampled.hip.y + rotated.y)
        };
        let arm = |d: &mut _, id: &str, socket: &str, rotation: f32| {
            let socket = body_frame.sockets[socket];
            let shoulder = at(body_point(socket[0], socket[1]), 0.0).position;
            self.pieces.draw(
                d,
                id,
                0,
                &PiecePose {
                    position: shoulder,
                    scale: depth,
                    flip: sign < 0.0,
                    rotation: (rotation + sampled.lean) * sign,
                    tint: Color::WHITE,
                },
            );
        };
        arm(d, &self.run.far_arm_piece, "far_shoulder", sampled.far_arm);
        for (leg, pose, far) in [
            (&self.run.far, sampled.far, true),
            (&self.run.near, sampled.near, false),
        ] {
            let tint = if far {
                Color::new(
                    self.mesh.far_tint[0],
                    self.mesh.far_tint[1],
                    self.mesh.far_tint[2],
                    255,
                )
            } else {
                Color::WHITE
            };
            let triangles: Vec<_> = self
                .mesh
                .triangles(pose)
                .into_iter()
                .map(|triangle| {
                    triangle.map(|v| TexturePoint {
                        position: at(v.position, 0.0).position,
                        uv: Vector2::new(v.uv.x, v.uv.y),
                    })
                })
                .collect();
            self.pieces
                .draw_triangles(d, &self.mesh.leg_piece, &triangles, tint);
            let mut boot = at(pose.ankle, pose.foot.rotation);
            boot.tint = tint;
            self.pieces.draw(d, &leg.boot, 0, &boot);
        }
        let mut triangles = Vec::new();
        let mut start = 0.0;
        for band in &self.mesh.body_bands {
            let vertex = |x: f32, y: f32| TexturePoint {
                position: at(body_point(x, y), 0.0).position,
                uv: Vector2::new(x / body_frame.source[2], y / body_frame.source[3]),
            };
            let a = vertex(0.0, start);
            let b = vertex(0.0, band.end);
            let c = vertex(body_frame.source[2], band.end);
            let e = vertex(body_frame.source[2], start);
            triangles.extend([[a, b, c], [a, c, e]]);
            start = band.end;
        }
        self.pieces
            .draw_triangles(d, &self.run.body_piece, &triangles, Color::WHITE);
        arm(
            d,
            &self.run.near_arm_piece,
            "near_shoulder",
            sampled.near_arm,
        );
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
