//! Advances authored chapter phases and resolves its small playable spaces.
//!
//! System: Adventure chapter gameplay. Input is consumed only in its current
//! phase; conversations and phone gestures cannot queue attacks for later.

use super::phone::PHONE_DONE_TICK;
use super::{
    Chapter, ChapterInput, CheckpointStage as C, INTRO_TICKS, Phase, SHOP_EXIT_CLEARANCE_TICKS,
    SHOP_SHUTTER_TICKS, Scene,
};
use crate::adventure::combat::{Action, Outcome};
use crate::math::vec2::Vec2;

impl Chapter {
    /// Advances one fixed update. Pausing means not calling this method.
    pub fn tick(&mut self, input: ChapterInput) {
        if input.retry
            && self.phase == Phase::PassageCombat
            && self.combat.outcome == Outcome::Defeat
        {
            self.retry();
            return;
        }
        if self.phase == Phase::Complete {
            return;
        }
        self.ticks = self.ticks.saturating_add(1);
        self.phase_ticks = self.phase_ticks.saturating_add(1);
        if let Some(age) = &mut self.neighbour_ticks {
            *age = age.saturating_add(1);
        }
        if let Some(age) = &mut self.residents_escape_ticks {
            *age = age.saturating_add(1);
        }
        if input.skip {
            self.skip_acting();
            self.update_camera();
            return;
        }
        match self.phase {
            Phase::Intro => {
                if input.advance || self.phase_ticks >= INTRO_TICKS {
                    self.enter(Phase::ExploreDriver);
                }
            }
            Phase::DriverApproach
            | Phase::ShopApproach
            | Phase::NeighbourApproach
            | Phase::LaneApproach => {
                let done = self.step_route();
                let door_ready =
                    self.phase != Phase::ShopApproach || self.phase_ticks >= SHOP_SHUTTER_TICKS;
                if (done && door_ready) || input.advance {
                    self.settle_route();
                    self.enter(match self.phase {
                        Phase::DriverApproach => Phase::DriverDialogue,
                        Phase::ShopApproach => Phase::ShopDialogue,
                        Phase::LaneApproach => Phase::LaneDialogue,
                        _ => Phase::NeighbourDialogue,
                    });
                }
            }
            Phase::DriverDialogue
            | Phase::ShopDialogue
            | Phase::NeighbourDialogue
            | Phase::LaneDialogue => {
                if input.advance {
                    self.advance_dialogue();
                }
            }
            Phase::DriverReturn | Phase::ShopReturn | Phase::NeighbourReturn => {
                let done = self.step_route();
                let door_ready = self.phase != Phase::ShopReturn
                    || self.phase_ticks >= SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS;
                if (done && door_ready) || input.advance {
                    self.finish_return();
                }
            }
            Phase::Phone => {
                if input.advance || self.phase_ticks >= PHONE_DONE_TICK {
                    self.enter(Phase::LeaveStreet);
                    self.save_at(C::PhoneDone);
                }
            }
            Phase::PassageCombat => {
                self.combat.tick(input.combat());
                self.clamp_bodies();
                if self.combat.outcome == Outcome::Victory {
                    self.enter(Phase::PassageClear);
                    self.save_at(C::PassageCleared);
                    self.residents_escape_ticks = Some(0);
                }
            }
            Phase::Departure => {
                if input.advance || self.phase_ticks >= 180 {
                    self.enter(Phase::Complete);
                    self.save_at(C::Complete);
                }
            }
            Phase::Complete => {}
            _ => self.explore(input),
        }
        self.update_camera();
    }

    fn explore(&mut self, input: ChapterInput) {
        if input.interact
            && let Some(interaction) = self.nearby_interaction()
        {
            match self.phase {
                Phase::ExploreDriver => self.begin_approach(interaction.id, Phase::DriverApproach),
                Phase::ExploreShop => self.begin_approach(interaction.id, Phase::ShopApproach),
                Phase::ExploreNeighbour => {
                    self.begin_approach(interaction.id, Phase::NeighbourApproach)
                }
                Phase::LaneExplore => self.begin_approach(interaction.id, Phase::LaneApproach),
                _ => {}
            }
            return;
        }
        self.explore_motion(input);
        let geometry = self.world.scene(self.scene);
        if self.phase == Phase::PassageExplore
            && geometry
                .poi("enemy")
                .expect("validated enemy")
                .region
                .contains(self.player().position)
        {
            self.combat.enemy_awake = true;
            self.combat.outcome = Outcome::Ongoing;
            self.enter(Phase::PassageCombat);
            self.save_at(C::PassageFight);
        } else if geometry.exit.contains(self.player().position) && self.player().grounded {
            match self.phase {
                Phase::LeaveStreet => {
                    self.begin_scene(Scene::Lane, Phase::LaneExplore, C::LaneStart)
                }
                Phase::LaneExit => {
                    self.begin_scene(Scene::Passage, Phase::PassageExplore, C::PassageStart)
                }
                Phase::PassageClear => self.enter(Phase::Departure),
                _ => {}
            }
        }
    }

    fn explore_motion(&mut self, input: ChapterInput) {
        let previous = self.player().position;
        self.combat.tick_exploration(input.combat());
        let geometry = self.world.scene(self.scene);
        let actor = &mut self.combat.player;
        let half_width = actor.hurtbox().width * 0.5;
        for obstacle in &geometry.obstacles {
            let solid = obstacle.rect();
            if !actor.hurtbox().intersects(solid) {
                continue;
            }
            if previous.y <= solid.y + 0.01 && actor.velocity.y >= 0.0 {
                actor.position.y = solid.y;
                actor.velocity.y = 0.0;
                actor.grounded = true;
            } else if previous.x + half_width <= solid.x + 0.01 {
                actor.position.x = solid.x - half_width;
                actor.velocity.x = 0.0;
            } else if previous.x - half_width >= solid.right() - 0.01 {
                actor.position.x = solid.right() + half_width;
                actor.velocity.x = 0.0;
            } else {
                // Handles an edited starting overlap deterministically instead
                // of allowing the body to tunnel through a solid rectangle.
                actor.position.x = if previous.x < solid.center_x() {
                    solid.x - half_width
                } else {
                    solid.right() + half_width
                };
                actor.velocity.x = 0.0;
            }
        }
        if actor.grounded && actor.position.y < geometry.floor_y {
            let supported = geometry.obstacles.iter().any(|r| {
                (actor.position.y - r.y).abs() < 0.01
                    && actor.position.x + half_width > r.x
                    && actor.position.x - half_width < r.x + r.width
            });
            if !supported {
                actor.grounded = false;
            }
        }
        self.clamp_bodies();
    }

    fn clamp_bodies(&mut self) {
        let geometry = self.world.scene(self.scene);
        for actor in [&mut self.combat.player, &mut self.combat.enemy] {
            let clamped = actor.position.x.clamp(geometry.walk_min, geometry.walk_max);
            if clamped != actor.position.x {
                actor.velocity.x = 0.0;
            }
            actor.position.x = clamped;
        }
    }

    fn advance_dialogue(&mut self) {
        self.dialogue_line += 1;
        self.phase_ticks = 0;
        if self.dialogue_line < 3 {
            return;
        }
        match self.phase {
            Phase::DriverDialogue => self.begin_return(Phase::DriverReturn),
            Phase::ShopDialogue => {
                self.neighbour_ticks = Some(0);
                self.begin_return(Phase::ShopReturn);
            }
            Phase::NeighbourDialogue => self.begin_return(Phase::NeighbourReturn),
            Phase::LaneDialogue => {
                self.enter(Phase::LaneExit);
                self.save_at(C::LaneCleared);
            }
            _ => {}
        }
    }

    fn finish_return(&mut self) {
        self.settle_route();
        self.combat.player.grounded = true;
        let (phase, checkpoint) = match self.phase {
            Phase::DriverReturn => (Phase::ExploreShop, C::DriverChecked),
            Phase::ShopReturn => {
                // An explicit skip settles the neighbour safely outside, too.
                self.neighbour_ticks = Some(self.neighbour_ticks.unwrap_or(0).max(90));
                (Phase::ExploreNeighbour, C::ShopChecked)
            }
            _ => (Phase::Phone, C::ContactReady),
        };
        self.enter(phase);
        self.save_at(checkpoint);
    }

    fn skip_acting(&mut self) {
        match self.phase {
            Phase::Intro => self.enter(Phase::ExploreDriver),
            Phase::DriverApproach
            | Phase::ShopApproach
            | Phase::NeighbourApproach
            | Phase::LaneApproach => {
                self.settle_route();
                self.enter(match self.phase {
                    Phase::DriverApproach => Phase::DriverDialogue,
                    Phase::ShopApproach => Phase::ShopDialogue,
                    Phase::LaneApproach => Phase::LaneDialogue,
                    _ => Phase::NeighbourDialogue,
                });
            }
            Phase::DriverDialogue
            | Phase::ShopDialogue
            | Phase::NeighbourDialogue
            | Phase::LaneDialogue => {
                self.dialogue_line = 2;
                self.advance_dialogue();
            }
            Phase::DriverReturn | Phase::ShopReturn | Phase::NeighbourReturn => {
                self.finish_return()
            }
            Phase::Phone => {
                self.combat.player.action = Action::Idle;
                self.combat.player.velocity = Vec2::ZERO;
                self.enter(Phase::LeaveStreet);
                self.save_at(C::PhoneDone);
            }
            Phase::Departure => {
                self.enter(Phase::Complete);
                self.save_at(C::Complete);
            }
            _ => {}
        }
    }
}
