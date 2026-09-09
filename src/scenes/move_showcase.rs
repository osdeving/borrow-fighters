//! Runs contextual, repeatable two-fighter demonstrations through the real match rules.
//!
//! System: Training scene. Scripted inputs arrange useful combat situations;
//! `World` alone applies collisions, damage, guard, reactions and pushback.

use crate::{
    audio::AudioEvent,
    characters::{CharacterBodyMetricsCatalog, CharacterId},
    combat::{
        fighter::{AttackKind, Facing, Fighter, FighterInput, PlayerSlot},
        move_data::move_spec,
    },
    config::{FIXED_TIMESTEP, FLOOR_Y, WINDOW_WIDTH, world_px},
    engine::sprites::SpriteManifest,
    game::{
        combat_log::CombatLogKind,
        world::{World, WorldSpriteCombatManifests},
    },
};

use super::combat_lab::{CombatLabInput, CombatLabMove};

const PREPARATION_FRAMES: u32 = 30;
/// Full playback includes long specials, airborne reactions and wake-up.
pub const SCENARIO_FRAMES: u32 = 260;
const DEFENSE_SCENARIOS: usize = 4;

/// Startup options for the contextual move showcase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MoveShowcaseOptions {
    pub character: CharacterId,
}

/// The concrete situation demonstrated by the current pair of fighters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShowcaseScenario {
    Attack(CombatLabMove),
    StandingBlock,
    OverheadBlock,
    CrouchingBlock,
    ProjectileBlock,
}

impl ShowcaseScenario {
    /// Attack used in this scenario, including the opponent's defense examples.
    pub const fn selected_move(self) -> CombatLabMove {
        match self {
            Self::Attack(selected) => selected,
            Self::StandingBlock => CombatLabMove::LightPunch,
            Self::OverheadBlock => CombatLabMove::Overhead,
            Self::CrouchingBlock => CombatLabMove::Sweep,
            Self::ProjectileBlock => CombatLabMove::Projectile,
        }
    }

    /// Whether the selected character demonstrates defense rather than offense.
    pub const fn is_defense(self) -> bool {
        !matches!(self, Self::Attack(_))
    }
}

/// Persisted contact result, obtained from the simulation's combat log.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShowcaseResult {
    #[default]
    Pending,
    Hit {
        damage: i32,
    },
    Blocked {
        damage: i32,
    },
    Whiff,
}

/// Deterministic contextual playback with the same combat state as a match.
#[derive(Clone, Debug)]
pub struct MoveShowcase {
    character: CharacterId,
    scenario_index: usize,
    world: World,
    body_metrics: CharacterBodyMetricsCatalog,
    combat_manifests: WorldSpriteCombatManifests,
    current_frame: u32,
    attack_started: bool,
    jump_started: bool,
    paused: bool,
    repeat_current: bool,
    sides_reversed: bool,
    result: ShowcaseResult,
}

impl Default for MoveShowcase {
    fn default() -> Self {
        Self::new(MoveShowcaseOptions::default())
    }
}

impl MoveShowcase {
    /// Creates a showcase beginning with a jab against an approaching opponent.
    pub fn new(options: MoveShowcaseOptions) -> Self {
        let mut scene = Self {
            character: options.character,
            scenario_index: 0,
            world: World::new_with_characters(options.character, opponent_for(options.character)),
            body_metrics: CharacterBodyMetricsCatalog::default(),
            combat_manifests: WorldSpriteCombatManifests::default(),
            current_frame: 0,
            attack_started: false,
            jump_started: false,
            paused: false,
            repeat_current: false,
            sides_reversed: false,
            result: ShowcaseResult::Pending,
        };
        scene.reset_current_move();
        scene
    }

    /// Handles one fixed step, including pause, true frame advance and navigation.
    pub fn update(&mut self, input: CombatLabInput) {
        if input.previous_move {
            self.scenario_index = (self.scenario_index + self.move_count() - 1) % self.move_count();
            self.reset_current_move();
        }
        if input.next_move {
            self.scenario_index = (self.scenario_index + 1) % self.move_count();
            self.reset_current_move();
        }
        if input.replay || input.reset {
            self.reset_current_move();
        }
        if input.pause_toggle {
            self.paused = !self.paused;
        }
        if self.paused && !input.step_frame {
            return;
        }
        if self.current_frame >= SCENARIO_FRAMES {
            if !self.repeat_current {
                self.scenario_index = (self.scenario_index + 1) % self.move_count();
            }
            self.reset_current_move();
            return;
        }
        let (player_one, player_two) = self.scenario_inputs();
        self.world.update(FIXED_TIMESTEP, player_one, player_two);
        self.current_frame += 1;
        self.update_result();
    }

    /// Returns the selected playable character.
    pub const fn character(&self) -> CharacterId {
        self.character
    }

    /// Returns the opponent, which rotates through the same public roster.
    pub fn opponent_character(&self) -> CharacterId {
        opponent_for(self.character)
    }

    /// Returns the current scenario; defense cases follow the offensive move list.
    pub fn scenario(&self) -> ShowcaseScenario {
        let moves = self.offensive_moves();
        if let Some(selected) = moves.get(self.scenario_index) {
            ShowcaseScenario::Attack(*selected)
        } else {
            match self.scenario_index - moves.len() {
                0 => ShowcaseScenario::StandingBlock,
                1 => ShowcaseScenario::OverheadBlock,
                2 => ShowcaseScenario::CrouchingBlock,
                _ => ShowcaseScenario::ProjectileBlock,
            }
        }
    }

    /// Selects one offensive move and starts its situation at frame zero.
    pub fn select_move(&mut self, selected: CombatLabMove) {
        if let Some(index) = self
            .offensive_moves()
            .iter()
            .position(|entry| *entry == selected)
        {
            self.scenario_index = index;
            self.reset_current_move();
        }
    }

    /// Selects an attack or defense example while retaining playback options.
    pub fn select_scenario(&mut self, scenario: ShowcaseScenario) {
        let defense_offset = match scenario {
            ShowcaseScenario::Attack(selected) => {
                self.select_move(selected);
                return;
            }
            ShowcaseScenario::StandingBlock => 0,
            ShowcaseScenario::OverheadBlock => 1,
            ShowcaseScenario::CrouchingBlock => 2,
            ShowcaseScenario::ProjectileBlock => 3,
        };
        self.scenario_index = self.offensive_moves().len() + defense_offset;
        self.reset_current_move();
    }

    /// Returns the attack family currently being demonstrated.
    pub fn selected_move(&self) -> CombatLabMove {
        self.scenario().selected_move()
    }

    /// Returns the one-based scenario number.
    pub const fn move_number(&self) -> usize {
        self.scenario_index + 1
    }

    /// Returns the number of offensive moves and defense demonstrations.
    pub fn move_count(&self) -> usize {
        self.offensive_moves().len() + DEFENSE_SCENARIOS
    }

    /// Exposes the complete simulation for rendering and evidence collection.
    pub const fn world(&self) -> &World {
        &self.world
    }

    /// Returns exact fixed-step playback time.
    pub const fn current_frame(&self) -> u32 {
        self.current_frame
    }

    /// Returns the latest actual collision result for this scenario.
    pub const fn result(&self) -> ShowcaseResult {
        self.result
    }

    /// Returns whether playback is paused.
    pub const fn paused(&self) -> bool {
        self.paused
    }

    /// Returns whether the actors are waiting before the demonstration.
    pub const fn resting(&self) -> bool {
        self.current_frame < PREPARATION_FRAMES
    }

    /// Returns whether the current example repeats instead of advancing.
    pub const fn repeat_current(&self) -> bool {
        self.repeat_current
    }

    /// Toggles repetition of the current example.
    pub fn toggle_repeat(&mut self) {
        self.repeat_current = !self.repeat_current;
    }

    /// Returns whether the selected character starts on the right.
    pub const fn sides_reversed(&self) -> bool {
        self.sides_reversed
    }

    /// Replays with both initial positions mirrored, preserving player identities.
    pub fn switch_sides(&mut self) {
        self.sides_reversed = !self.sides_reversed;
        self.reset_current_move();
    }

    /// Uses the same loaded physical proportions as normal matches.
    pub fn set_body_metrics(&mut self, metrics: CharacterBodyMetricsCatalog) {
        self.body_metrics = metrics;
        self.reset_current_move();
    }

    /// Keeps both baseline manifests attached across navigation and automatic resets.
    pub fn set_sprite_combat_manifests(&mut self, manifests: WorldSpriteCombatManifests) {
        self.combat_manifests = manifests;
        self.reset_current_move();
    }

    /// Compatibility helper for callers providing only the selected fighter metadata.
    pub fn set_combat_manifest(&mut self, manifest: Option<SpriteManifest>) {
        self.combat_manifests.player_one = manifest;
        self.reset_current_move();
    }

    /// Drains real contact and move audio without duplicating simulation events.
    pub fn take_audio_events(&mut self) -> Vec<AudioEvent> {
        self.world.take_audio_events()
    }

    /// Names the actual loadout move, including the opponent's attack in guard examples.
    pub fn move_label(&self) -> &'static str {
        let selected = self.selected_move();
        let Some(kind) = selected.attack_kind() else {
            return selected.label();
        };
        let actor = if self.scenario().is_defense() {
            &self.world.player_two
        } else {
            &self.world.player_one
        };
        actor
            .move_ids()
            .iter()
            .copied()
            .find(|id| AttackKind::from_move_id(*id) == kind)
            .map_or(selected.label(), |id| move_spec(id).label)
    }

    /// Concise situation label displayed above the stage.
    pub fn scenario_label(&self) -> &'static str {
        match self.scenario() {
            ShowcaseScenario::StandingBlock => "Standing defense / mid strike",
            ShowcaseScenario::OverheadBlock => "Standing defense / overhead",
            ShowcaseScenario::CrouchingBlock => "Crouching defense / low sweep",
            ShowcaseScenario::ProjectileBlock => "Standing defense / projectile",
            ShowcaseScenario::Attack(selected) => match selected {
                CombatLabMove::Sweep => "Sweep / punish standing guard",
                CombatLabMove::Overhead => "Overhead / punish crouching guard",
                CombatLabMove::Throw => "Back throw / punish guard and switch sides",
                CombatLabMove::AntiAir => "Anti-air / intercept a real jump-in",
                CombatLabMove::AirPunch | CombatLabMove::AirKick => {
                    "Jump attack / strike a grounded opponent"
                }
                CombatLabMove::Projectile => "Projectile / control distance",
                CombatLabMove::CinematicSpecial => "Cinematic special / commit at close range",
                CombatLabMove::SignatureSpecial => match self.character {
                    CharacterId::Rust => "Borrow Fortress / armored shield rush",
                    CharacterId::Duke => "System.out.println / three-wave paper barrage",
                    CharacterId::C => "Segmentation Fault / rupture beneath high guard",
                    CharacterId::Python => "import antigravity / serpent launch vortex",
                    CharacterId::Cpp => "Undefined Bazooka / explosive foot punishment",
                    CharacterId::Go => "Signature special",
                },
                _ => "Ground strike / stop an approaching opponent",
            },
        }
    }

    /// Explains the attack/guard relationship without exposing engine details.
    pub fn scenario_description(&self) -> &'static str {
        match self.scenario() {
            ShowcaseScenario::StandingBlock => "Hold guard standing to defend mid strikes.",
            ShowcaseScenario::OverheadBlock => "Stand and guard: overheads beat crouching defense.",
            ShowcaseScenario::CrouchingBlock => "Hold down + guard to defend low attacks.",
            ShowcaseScenario::ProjectileBlock => {
                "Guard absorbs the projectile; reduced chip damage remains."
            }
            ShowcaseScenario::Attack(CombatLabMove::Throw) => {
                "Grab a guarding opponent, lift, throw behind you and switch sides. Jump to escape the grab."
            }
            ShowcaseScenario::Attack(CombatLabMove::Sweep) => {
                "The opponent guards high. Attack the exposed legs."
            }
            ShowcaseScenario::Attack(CombatLabMove::Overhead) => {
                "The opponent guards low. Strike from above."
            }
            ShowcaseScenario::Attack(CombatLabMove::AntiAir) => {
                "Intercept the jump-in with an uppercut: the hit launches the opponent into a helpless fall."
            }
            ShowcaseScenario::Attack(CombatLabMove::AirPunch | CombatLabMove::AirKick) => {
                "Jump forward, then attack while descending into range."
            }
            ShowcaseScenario::Attack(CombatLabMove::Projectile) => {
                "Launch from distance; the opponent walks into the projectile."
            }
            ShowcaseScenario::Attack(CombatLabMove::CinematicSpecial) => {
                "The whole arena transforms. Get close to land the strike; guard or interrupt the wind-up."
            }
            ShowcaseScenario::Attack(CombatLabMove::SignatureSpecial) => match self.character {
                CharacterId::Rust => {
                    "Raise the safety shield and charge. Frontal protection has a short window; throws beat it."
                }
                CharacterId::Duke => {
                    "Verbosity made physical: three paper waves. Guard, jump, or punish the printer jam."
                }
                CharacterId::C => {
                    "Slam the book: corrupted memory erupts under the opponent. Guard low or jump away."
                }
                CharacterId::Python => {
                    "Import antigravity, levitate, and launch the opponent with a giant serpent vortex."
                }
                CharacterId::Cpp => {
                    "Bring an oversized bazooka to a foot fight. Guard low; punish the enormous recoil."
                }
                CharacterId::Go => "One signature per playable fighter; no meter required.",
            },
            _ => "Let the opponent advance into reach, then strike once and watch the reaction.",
        }
    }

    fn offensive_moves(&self) -> &'static [CombatLabMove] {
        const GO_MOVES: [CombatLabMove; 11] = [
            CombatLabMove::LightPunch,
            CombatLabMove::HeavyPunch,
            CombatLabMove::Kick,
            CombatLabMove::Sweep,
            CombatLabMove::Overhead,
            CombatLabMove::AntiAir,
            CombatLabMove::AirPunch,
            CombatLabMove::AirKick,
            CombatLabMove::Throw,
            CombatLabMove::Projectile,
            CombatLabMove::CinematicSpecial,
        ];
        if self.character == CharacterId::Go {
            &GO_MOVES
        } else {
            &CombatLabMove::ALL
        }
    }

    fn reset_current_move(&mut self) {
        self.world = World::new_with_character_body_metrics(
            self.character,
            self.opponent_character(),
            &self.body_metrics,
        );
        self.world
            .set_sprite_combat_manifests(self.combat_manifests.clone());
        let gap = match self.selected_move() {
            CombatLabMove::Projectile => world_px(300.0),
            CombatLabMove::CinematicSpecial => world_px(60.0),
            CombatLabMove::SignatureSpecial if self.character == CharacterId::Duke => {
                world_px(220.0)
            }
            CombatLabMove::SignatureSpecial if self.character == CharacterId::Rust => {
                // Leave the fortress visible before the two advancing bodies
                // meet; this spacing changes presentation, not combat reach.
                world_px(300.0)
            }
            CombatLabMove::SignatureSpecial if self.character == CharacterId::Python => {
                world_px(230.0)
            }
            CombatLabMove::AntiAir | CombatLabMove::AirPunch | CombatLabMove::AirKick => {
                world_px(150.0)
            }
            CombatLabMove::SignatureSpecial => world_px(90.0),
            _ => world_px(105.0),
        };
        let left = (WINDOW_WIDTH as f32
            - self.world.player_one.body_rect().width
            - self.world.player_two.body_rect().width
            - gap)
            * 0.5;
        if self.sides_reversed {
            self.world.player_two.position.x = left;
            self.world.player_one.position.x = left + self.world.player_two.body_rect().width + gap;
        } else {
            self.world.player_one.position.x = left;
            self.world.player_two.position.x = left + self.world.player_one.body_rect().width + gap;
        }
        self.world.player_one.face_toward(&self.world.player_two);
        self.world.player_two.face_toward(&self.world.player_one);
        self.current_frame = 0;
        self.attack_started = false;
        self.jump_started = false;
        self.result = ShowcaseResult::Pending;
    }

    fn scenario_inputs(&mut self) -> (FighterInput, FighterInput) {
        let scenario = self.scenario();
        let selected = scenario.selected_move();
        let (attacker, defender) = if scenario.is_defense() {
            (&self.world.player_two, &self.world.player_one)
        } else {
            (&self.world.player_one, &self.world.player_two)
        };
        let mut attack_input = FighterInput::default();
        let mut defend_input = FighterInput::default();
        let low_guard = matches!(
            scenario,
            ShowcaseScenario::CrouchingBlock | ShowcaseScenario::Attack(CombatLabMove::Overhead)
        );
        let high_guard = scenario.is_defense()
            || matches!(selected, CombatLabMove::Sweep | CombatLabMove::Throw)
            || selected == CombatLabMove::SignatureSpecial
                && matches!(self.character, CharacterId::C | CharacterId::Cpp);
        defend_input.block = high_guard || low_guard;
        defend_input.crouch = low_guard;
        if self.current_frame >= PREPARATION_FRAMES && self.result == ShowcaseResult::Pending {
            let gap = body_gap(attacker, defender);
            let anti_air = selected == CombatLabMove::AntiAir;
            let air_attack = matches!(selected, CombatLabMove::AirPunch | CombatLabMove::AirKick);
            if anti_air {
                defend_input = toward(defender);
                if !self.jump_started {
                    defend_input.jump = true;
                    self.jump_started = true;
                }
                if !self.attack_started && !defender.grounded && gap < world_px(50.0) {
                    attack_input = attack_for(selected, attacker.facing);
                    self.attack_started = true;
                }
            } else if air_attack {
                if gap > world_px(14.0) {
                    attack_input = toward(attacker);
                }
                if !self.jump_started {
                    attack_input.jump = true;
                    self.jump_started = true;
                }
                let altitude = FLOOR_Y - attacker.body_rect().bottom();
                if !self.attack_started
                    && !attacker.grounded
                    && attacker.velocity.y > 0.0
                    && altitude < world_px(110.0)
                    && gap < world_px(45.0)
                {
                    attack_input = attack_for(selected, attacker.facing);
                    self.attack_started = true;
                }
            } else if matches!(
                selected,
                CombatLabMove::SignatureSpecial | CombatLabMove::CinematicSpecial
            ) {
                // Stage each language's actual reach. Low blasts punish high
                // guard; the other examples let the opponent advance into range.
                if !self.attack_started {
                    attack_input = attack_for(selected, attacker.facing);
                    self.attack_started = true;
                }
                if !defend_input.block && gap > world_px(35.0) {
                    defend_input = toward(defender);
                }
            } else if selected == CombatLabMove::Projectile {
                if !defend_input.block {
                    defend_input = toward(defender);
                }
                if !self.attack_started {
                    attack_input = attack_for(selected, attacker.facing);
                    self.attack_started = true;
                }
            } else {
                if defend_input.block {
                    if !self.attack_started && gap > world_px(12.0) {
                        attack_input = toward(attacker);
                    }
                } else {
                    defend_input = toward(defender);
                }
                let trigger_gap = world_px(18.0);
                if !self.attack_started && gap <= trigger_gap {
                    attack_input = attack_for(selected, attacker.facing);
                    self.attack_started = true;
                }
            }
        }
        if scenario.is_defense() {
            (defend_input, attack_input)
        } else {
            (attack_input, defend_input)
        }
    }

    fn update_result(&mut self) {
        let expected_attacker = if self.scenario().is_defense() {
            PlayerSlot::Two
        } else {
            PlayerSlot::One
        };
        let mut total_damage = 0;
        let mut contact_count = 0;
        let mut all_blocked = true;
        for event in self.world.combat_log() {
            match event.kind {
                CombatLogKind::CloseAttackResolved {
                    attacker,
                    damage,
                    blocked,
                    ..
                }
                | CombatLogKind::ProjectileResolved {
                    attacker,
                    damage,
                    blocked,
                    ..
                } if attacker == expected_attacker => {
                    total_damage += damage;
                    contact_count += 1;
                    all_blocked &= blocked;
                }
                _ => {}
            }
        }
        if contact_count > 0 {
            self.result = if all_blocked {
                ShowcaseResult::Blocked {
                    damage: total_damage,
                }
            } else {
                ShowcaseResult::Hit {
                    damage: total_damage,
                }
            };
        } else if self.current_frame >= SCENARIO_FRAMES - 40 {
            self.result = ShowcaseResult::Whiff;
        }
    }
}

fn body_gap(a: &Fighter, b: &Fighter) -> f32 {
    if a.body_rect().center_x() < b.body_rect().center_x() {
        b.body_rect().x - a.body_rect().right()
    } else {
        a.body_rect().x - b.body_rect().right()
    }
}

fn toward(fighter: &Fighter) -> FighterInput {
    FighterInput {
        right: fighter.facing == Facing::Right,
        left: fighter.facing == Facing::Left,
        ..FighterInput::default()
    }
}

fn attack_for(selected: CombatLabMove, facing: Facing) -> FighterInput {
    let mut input = FighterInput::default();
    match selected {
        CombatLabMove::LightPunch | CombatLabMove::AirPunch => input.light_punch = true,
        CombatLabMove::HeavyPunch => input.heavy_punch = true,
        CombatLabMove::Kick | CombatLabMove::AirKick => input.kick = true,
        CombatLabMove::Sweep => {
            input.crouch = true;
            input.kick = true;
        }
        CombatLabMove::Overhead => {
            input.heavy_punch = true;
            input.right = facing == Facing::Right;
            input.left = facing == Facing::Left;
        }
        CombatLabMove::AntiAir => {
            input.heavy_punch = true;
            input.crouch = true;
        }
        CombatLabMove::Throw => {
            input.block = true;
            input.light_punch = true;
        }
        CombatLabMove::Projectile => input.projectile = true,
        CombatLabMove::SignatureSpecial => input.signature_special = true,
        CombatLabMove::CinematicSpecial => input.cinematic_special = true,
    }
    input
}

fn opponent_for(character: CharacterId) -> CharacterId {
    match character {
        CharacterId::Rust => CharacterId::Duke,
        CharacterId::Duke => CharacterId::C,
        CharacterId::C => CharacterId::Python,
        CharacterId::Python => CharacterId::Cpp,
        CharacterId::Cpp | CharacterId::Go => CharacterId::Rust,
    }
}
