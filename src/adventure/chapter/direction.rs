//! Samples dialogue, neighbours, camera and Rust's authored approach paths.
//!
//! System: Adventure chapter direction. Paths translate the same physical body
//! used in exploration, preserving its support, depth and return position.

use super::{
    Chapter, Dialogue, INTRO_TICKS, Interaction, Phase, ResidentView, SHOP_EXIT_CLEARANCE_TICKS,
    SHOP_SHUTTER_TICKS, Scene,
};
use crate::adventure::combat::{Action, Facing};
use crate::math::vec2::Vec2;

const DRIVER: [(&str, &str); 3] = [
    ("speaker.rust", "driver.0"),
    ("speaker.driver", "driver.1"),
    ("speaker.rust", "driver.2"),
];
const SHOP: [(&str, &str); 3] = [
    ("speaker.rust", "shop.0"),
    ("speaker.shopkeeper", "shop.1"),
    ("speaker.rust", "shop.2"),
];
const NEIGHBOUR: [(&str, &str); 3] = [
    ("speaker.neighbour", "neighbour.0"),
    ("speaker.neighbour", "neighbour.1"),
    ("speaker.rust", "neighbour.2"),
];
const LANE: [(&str, &str); 3] = [
    ("speaker.lane_resident", "lane.0"),
    ("speaker.rust", "lane.1"),
    ("speaker.lane_resident", "lane.2"),
];

impl Chapter {
    /// Current subtitle line; the root owns typography and external text loading.
    pub fn active_dialogue(&self) -> Option<Dialogue> {
        let lines: &[(&str, &str)] = match self.phase {
            Phase::Intro => &[("speaker.rust", "chapter.intro")],
            Phase::DriverDialogue => &DRIVER,
            Phase::ShopDialogue => &SHOP,
            Phase::NeighbourDialogue => &NEIGHBOUR,
            Phase::LaneDialogue => &LANE,
            Phase::Departure => &[
                ("speaker.rust", "departure.0"),
                ("speaker.rust", "departure.1"),
            ],
            _ => return None,
        };
        let line_index = if self.phase == Phase::Departure {
            (self.phase_ticks / 90).min(1) as usize
        } else {
            self.dialogue_line.min(lines.len() - 1)
        };
        Some(Dialogue {
            speaker: lines[line_index].0,
            key: lines[line_index].1,
            line_index,
            line_count: lines.len(),
        })
    }

    /// Interaction is available only at the correct reachable POI and while grounded.
    pub fn nearby_interaction(&self) -> Option<Interaction> {
        if !self.player().grounded
            || !matches!(
                self.player().action,
                Action::Idle | Action::Walk | Action::Block
            )
        {
            return None;
        }
        let (id, prompt_key) = match self.phase {
            Phase::ExploreDriver => ("driver", "interaction.driver"),
            Phase::ExploreShop => ("shop", "interaction.shop"),
            Phase::ExploreNeighbour if self.neighbour_ticks.unwrap_or(0) >= 90 => {
                ("neighbour", "interaction.neighbour")
            }
            Phase::LaneExplore => ("lane_resident", "interaction.lane_resident"),
            _ => return None,
        };
        self.world
            .scene(self.scene)
            .poi(id)?
            .region
            .contains(self.player().position)
            .then_some(Interaction { id, prompt_key })
    }

    /// Raised coverage during shop approach, open while speaking, closed after leaving.
    pub fn shutter_progress(&self) -> f32 {
        let fraction = (self.phase_ticks as f32 / SHOP_SHUTTER_TICKS as f32).min(1.0);
        match self.phase {
            Phase::ShopApproach => 1.0 - fraction,
            Phase::ShopDialogue => 0.0,
            Phase::ShopReturn => (self.phase_ticks.saturating_sub(SHOP_EXIT_CLEARANCE_TICKS)
                as f32
                / SHOP_SHUTTER_TICKS as f32)
                .min(1.0),
            _ => 1.0,
        }
    }

    /// Decorative people, including the neighbour's short walk out of the shop.
    pub fn residents(&self) -> Vec<ResidentView> {
        let speaking = self.active_dialogue().map(|line| line.speaker);
        let person = |id, position, facing, visible, walking, action_ticks| ResidentView {
            id,
            position,
            facing,
            visible,
            walking,
            action_ticks,
            talking: speaking
                == Some(match id {
                    "driver" => "speaker.driver",
                    "shopkeeper" => "speaker.shopkeeper",
                    "neighbour" => "speaker.neighbour",
                    _ => "speaker.lane_resident",
                }),
        };
        let geometry = self.world.scene(self.scene);
        match self.scene {
            Scene::Street => {
                let shop = geometry.poi("shop").expect("validated shop").position.vec();
                let neighbour = geometry
                    .poi("neighbour")
                    .expect("validated neighbour")
                    .position
                    .vec();
                let age = self.neighbour_ticks.unwrap_or(0);
                let progress = (age as f32 / 90.0).min(1.0);
                vec![
                    person(
                        "driver",
                        geometry
                            .poi("driver")
                            .expect("validated driver")
                            .position
                            .vec(),
                        Facing::Left,
                        true,
                        false,
                        self.phase_ticks,
                    ),
                    person(
                        "shopkeeper",
                        shop,
                        Facing::Left,
                        self.shutter_progress() < 0.98,
                        false,
                        self.phase_ticks,
                    ),
                    person(
                        "neighbour",
                        Vec2::new(
                            shop.x + (neighbour.x - shop.x) * progress,
                            shop.y + (neighbour.y - shop.y) * progress,
                        ),
                        if age < 90 {
                            Facing::Right
                        } else {
                            Facing::Left
                        },
                        self.neighbour_ticks.is_some(),
                        age < 90,
                        age,
                    ),
                ]
            }
            Scene::Lane => vec![person(
                "lane_resident",
                geometry
                    .poi("lane_resident")
                    .expect("validated resident")
                    .position
                    .vec(),
                Facing::Left,
                true,
                false,
                self.phase_ticks,
            )],
            Scene::Passage => {
                let age = self.residents_escape_ticks.unwrap_or(0);
                let progress = (age as f32 / 300.0).min(1.0);
                // These residents wait safely behind Rust. After the encounter
                // they leave through the same passage he has actually cleared.
                [0, 1]
                    .into_iter()
                    .map(|index| {
                        person(
                            if index == 0 {
                                "passage_resident_0"
                            } else {
                                "passage_resident_1"
                            },
                            Vec2::new(
                                110.0 + index as f32 * 70.0 + progress * (geometry.width + 80.0),
                                580.0,
                            ),
                            Facing::Right,
                            age < 300,
                            self.residents_escape_ticks.is_some(),
                            age,
                        )
                    })
                    .collect()
            }
        }
    }

    pub(super) fn begin_approach(&mut self, id: &str, phase: Phase) {
        let mut route = vec![self.player().position];
        route.extend(
            self.world
                .scene(self.scene)
                .poi(id)
                .expect("validated interaction")
                .path
                .iter()
                .map(|p| p.vec()),
        );
        self.return_route = route.iter().rev().copied().collect();
        self.route = route;
        self.route_index = 1;
        self.enter(phase);
    }

    pub(super) fn begin_return(&mut self, phase: Phase) {
        self.route = self.return_route.clone();
        self.route_index = 1;
        self.enter(phase);
    }

    /// Chooses an actual back/front view from the next authored route segment.
    pub fn travel_view(&self) -> crate::adventure::locomotion::TravelView {
        use crate::adventure::locomotion::TravelView;
        if self.player().action != Action::Walk || self.route_index >= self.route.len() {
            return TravelView::Side;
        }
        let target = self.route[self.route_index];
        TravelView::from_delta(
            target.x - self.player().position.x,
            target.y - self.player().position.y,
        )
    }

    /// Constant speed along authored segments, without relocating at a phase join.
    pub(super) fn step_route(&mut self) -> bool {
        self.combat
            .player
            .set_gait(crate::adventure::locomotion::Gait::Walk);
        let mut remaining: f32 = 225.0 / 60.0;
        while let Some(target) = self.route.get(self.route_index).copied() {
            let delta = Vec2::new(
                target.x - self.player().position.x,
                target.y - self.player().position.y,
            );
            let distance = delta.x.hypot(delta.y);
            if delta.x.abs() > 0.001 {
                self.combat.player.facing = if delta.x < 0.0 {
                    Facing::Left
                } else {
                    Facing::Right
                };
            }
            if distance <= remaining {
                self.combat.player.stride_distance += distance / self.player_scale();
                self.combat.player.position = target;
                self.route_index += 1;
                remaining -= distance;
                if remaining <= 0.001 {
                    break;
                }
            } else {
                self.combat.player.stride_distance += remaining / self.player_scale();
                self.combat.player.position.x += delta.x / distance * remaining;
                self.combat.player.position.y += delta.y / distance * remaining;
                break;
            }
        }
        let complete = self.route_index >= self.route.len();
        self.combat.player.action = if complete { Action::Idle } else { Action::Walk };
        self.combat.player.action_ticks = self.combat.player.action_ticks.saturating_add(1);
        complete
    }

    pub(super) fn settle_route(&mut self) {
        if let Some(last) = self.route.last() {
            self.combat.player.position = *last;
        }
        self.route_index = self.route.len();
        self.combat.player.velocity = Vec2::ZERO;
        self.combat.player.action = Action::Idle;
    }

    pub(super) fn update_camera(&mut self) {
        if self.phase == Phase::Intro {
            let t =
                (self.phase_ticks.saturating_sub(120) as f32 / (INTRO_TICKS - 120) as f32).min(1.0);
            let smooth = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
            self.camera.zoom = 1.5 - 0.5 * smooth;
            let half = 640.0 / self.camera.zoom;
            let width = self.world.scene(self.scene).width;
            let detail_x = self
                .world
                .scene(self.scene)
                .poi("driver")
                .expect("validated driver")
                .position
                .x;
            self.camera.target = Vec2::new(
                (detail_x + (self.player().position.x - detail_x) * smooth)
                    .clamp(half, width - half),
                340.0 + 20.0 * smooth,
            );
            return;
        }
        let close = matches!(
            self.phase,
            Phase::DriverApproach
                | Phase::DriverDialogue
                | Phase::ShopApproach
                | Phase::ShopDialogue
                | Phase::NeighbourApproach
                | Phase::NeighbourDialogue
                | Phase::Phone
                | Phase::LaneApproach
                | Phase::LaneDialogue
        );
        let desired_zoom = if close { 1.10 } else { 1.0 };
        self.camera.zoom += (desired_zoom - self.camera.zoom) * 0.035;
        let half_width = 640.0 / self.camera.zoom;
        let width = self.world.scene(self.scene).width;
        let desired_x = self
            .player()
            .position
            .x
            .clamp(half_width, width - half_width);
        self.camera.target.x += (desired_x - self.camera.target.x) * 0.05;
        self.camera.target.x = self.camera.target.x.clamp(half_width, width - half_width);
        self.camera.target.y = 360.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adventure::chapter::{ChapterInput, Checkpoint, CheckpointStage, World};

    #[test]
    fn crossing_shows_the_back_going_to_the_shop_and_front_on_the_same_return_route() {
        use crate::adventure::locomotion::TravelView;
        let mut chapter = Chapter::new(World::bundled(), false);
        let shop = chapter.world.scene(Scene::Street).poi("shop").unwrap();
        chapter.combat.player.position = shop.path[0].vec();
        let start = chapter.player().position;
        chapter.begin_approach("shop", Phase::ShopApproach);
        let mut saw_back = false;
        while !chapter.step_route() {
            let view = chapter.travel_view();
            saw_back |= view == TravelView::Back;
            assert_ne!(view, TravelView::Front);
        }
        assert!(saw_back);
        assert_eq!(
            chapter.travel_view(),
            TravelView::Side,
            "dialogue uses its authored body"
        );
        chapter.begin_return(Phase::ShopReturn);
        let mut saw_front = false;
        while !chapter.step_route() {
            let view = chapter.travel_view();
            saw_front |= view == TravelView::Front;
            assert_ne!(view, TravelView::Back);
        }
        assert!(saw_front);
        assert_eq!(chapter.player().position, start);
        assert_eq!(chapter.travel_view(), TravelView::Side);
        assert_eq!(TravelView::from_delta(50.0, 0.0), TravelView::Side);
        assert_eq!(TravelView::from_delta(-50.0, 0.0), TravelView::Side);
    }

    #[test]
    fn closing_shutter_waits_until_the_neighbours_full_pose_clears_the_jamb() {
        let mut chapter = Chapter::from_checkpoint(
            World::bundled(),
            Checkpoint {
                version: 1,
                stage: CheckpointStage::DriverChecked,
                prologue_played_victory: false,
            },
        )
        .unwrap();
        let shop = chapter.world.scene(Scene::Street).poi("shop").unwrap();
        let doorway_x = shop.position.x;
        chapter.combat.player.position = shop.path[0].vec();
        chapter.tick(ChapterInput {
            interact: true,
            ..Default::default()
        });
        for _ in 0..180 {
            chapter.tick(ChapterInput::default());
        }
        assert_eq!(chapter.phase, Phase::ShopDialogue);
        for _ in 0..3 {
            chapter.tick(ChapterInput {
                advance: true,
                ..Default::default()
            });
        }
        for age in 0..SHOP_EXIT_CLEARANCE_TICKS + SHOP_SHUTTER_TICKS {
            assert_eq!(chapter.phase, Phase::ShopReturn);
            assert_eq!(chapter.phase_ticks, age);
            if chapter.shutter_progress() > 0.0 {
                let neighbour = chapter
                    .residents()
                    .into_iter()
                    .find(|person| person.id == "neighbour")
                    .unwrap();
                assert!(
                    neighbour.position.x - 40.0 > doorway_x + 97.0 * 0.5,
                    "The full running sprite must leave the opening before it closes"
                );
            } else {
                assert!(age <= SHOP_EXIT_CLEARANCE_TICKS);
            }
            chapter.tick(ChapterInput::default());
        }
        assert_eq!(chapter.phase, Phase::ExploreNeighbour);
        assert_eq!(chapter.shutter_progress(), 1.0);
    }
}
