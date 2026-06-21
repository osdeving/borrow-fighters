//! Owns the isolated Combat Lab scene state.
//!
//! System: Combat Lab scene. This file owns isolated move playback state and
//! intentionally keeps Raylib drawing outside the testable lab model.
//!
//! The lab reuses combat primitives without match flow so move timing, pivots,
//! hitboxes, hurtboxes, and projectile spawn can be inspected directly.

use crate::characters::{CharacterId, character_spec};
use crate::combat::{
    fighter::{AttackKind, Fighter, FighterInput, PlayerSlot},
    frame::FrameCount,
    move_data::MoveInputKind,
    projectile::Projectile,
};
use crate::config::{FIXED_TIMESTEP, WINDOW_WIDTH};
use crate::math::rect::Rect;

use super::combat_lab_analysis::{
    CombatLabAdvantage, CombatLabAnalysisMove, analyze_advantage, contact_dummy_body,
    default_dummy_body,
};

const LAB_FIGHTER_X: f32 = 430.0;
const LAB_JUMP_PREVIEW_HEIGHT: f32 = 92.0;

/// Move selected for isolated playback.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CombatLabMove {
    #[default]
    LightPunch,
    HeavyPunch,
    Kick,
    Projectile,
}

impl CombatLabMove {
    /// Ordered move list used by cycling controls.
    pub const ALL: [Self; 4] = [
        Self::LightPunch,
        Self::HeavyPunch,
        Self::Kick,
        Self::Projectile,
    ];

    /// Parses a CLI move name.
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "light_punch" | "light-punch" | "lp" | "jab" => Some(Self::LightPunch),
            "heavy_punch" | "heavy-punch" | "hp" => Some(Self::HeavyPunch),
            "kick" | "k" => Some(Self::Kick),
            "projectile" | "special" | "fireball" => Some(Self::Projectile),
            _ => None,
        }
    }

    /// Returns the move label used by the lab overlay.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LightPunch => "LP",
            Self::HeavyPunch => "HP",
            Self::Kick => "Kick",
            Self::Projectile => "Projectile",
        }
    }

    /// Returns the close attack kind for strike moves.
    pub const fn attack_kind(self) -> Option<AttackKind> {
        match self {
            Self::LightPunch => Some(AttackKind::LightPunch),
            Self::HeavyPunch => Some(AttackKind::HeavyPunch),
            Self::Kick => Some(AttackKind::Kick),
            Self::Projectile => None,
        }
    }

    const fn analysis_move(self) -> CombatLabAnalysisMove {
        match self {
            Self::LightPunch => CombatLabAnalysisMove::Close(MoveInputKind::LightPunch),
            Self::HeavyPunch => CombatLabAnalysisMove::Close(MoveInputKind::HeavyPunch),
            Self::Kick => CombatLabAnalysisMove::Close(MoveInputKind::Kick),
            Self::Projectile => CombatLabAnalysisMove::Projectile,
        }
    }

    fn first_frame_input(self) -> FighterInput {
        match self {
            Self::LightPunch => FighterInput {
                light_punch: true,
                ..FighterInput::default()
            },
            Self::HeavyPunch => FighterInput {
                heavy_punch: true,
                ..FighterInput::default()
            },
            Self::Kick => FighterInput {
                kick: true,
                ..FighterInput::default()
            },
            Self::Projectile => FighterInput::default(),
        }
    }
}

/// Static pose or move playback mode selected in the Combat Lab.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CombatLabPose {
    #[default]
    Move,
    Idle,
    Crouch,
    Jump,
    Block,
    Hit,
    Victory,
}

impl CombatLabPose {
    /// Ordered pose list used by cycling controls.
    pub const ALL: [Self; 7] = [
        Self::Move,
        Self::Idle,
        Self::Crouch,
        Self::Jump,
        Self::Block,
        Self::Hit,
        Self::Victory,
    ];

    /// Parses a CLI pose name.
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "move" | "attack" | "playback" => Some(Self::Move),
            "idle" | "stand" | "standing" => Some(Self::Idle),
            "crouch" | "duck" | "down" => Some(Self::Crouch),
            "jump" | "air" | "airborne" => Some(Self::Jump),
            "block" | "guard" => Some(Self::Block),
            "hit" | "hurt" => Some(Self::Hit),
            "victory" | "taunt" | "win" => Some(Self::Victory),
            _ => None,
        }
    }

    /// Returns the short label used by the lab overlay.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Idle => "idle",
            Self::Crouch => "crouch",
            Self::Jump => "jump",
            Self::Block => "block",
            Self::Hit => "hit",
            Self::Victory => "victory",
        }
    }

    const fn is_move_playback(self) -> bool {
        matches!(self, Self::Move)
    }
}

/// Startup options for the Combat Lab.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CombatLabOptions {
    pub character: CharacterId,
    pub selected_move: CombatLabMove,
    pub pose: CombatLabPose,
}

/// Input commands consumed by the Combat Lab scene.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CombatLabInput {
    pub next_move: bool,
    pub previous_move: bool,
    pub replay: bool,
    pub pause_toggle: bool,
    pub step_frame: bool,
    pub reset: bool,
    pub next_pose: bool,
    pub previous_pose: bool,
    pub toggle_hurtboxes: bool,
    pub toggle_hitboxes: bool,
    pub toggle_pivot: bool,
    pub toggle_dummy: bool,
}

/// Isolated move playback state for combat tuning.
#[derive(Clone, Debug)]
pub struct CombatLab {
    character: CharacterId,
    selected_move: CombatLabMove,
    pose: CombatLabPose,
    fighter: Fighter,
    projectiles: Vec<Projectile>,
    current_frame: FrameCount,
    paused: bool,
    show_hurtboxes: bool,
    show_hitboxes: bool,
    show_pivot: bool,
    show_dummy: bool,
}

impl Default for CombatLab {
    fn default() -> Self {
        Self::new(CombatLabOptions::default())
    }
}

impl CombatLab {
    /// Creates a Combat Lab scene with an isolated fighter.
    pub fn new(options: CombatLabOptions) -> Self {
        let mut lab = Self {
            character: options.character,
            selected_move: options.selected_move,
            pose: options.pose,
            fighter: fighter_for(options.character),
            projectiles: Vec::new(),
            current_frame: FrameCount::ZERO,
            paused: false,
            show_hurtboxes: true,
            show_hitboxes: true,
            show_pivot: true,
            show_dummy: false,
        };
        lab.reset_playback();
        lab
    }

    /// Handles one fixed lab tick.
    pub fn update(&mut self, input: CombatLabInput) {
        if input.previous_move {
            self.select_previous_move();
        }
        if input.next_move {
            self.select_next_move();
        }
        if input.previous_pose {
            self.select_previous_pose();
        }
        if input.next_pose {
            self.select_next_pose();
        }
        if input.replay || input.reset {
            self.reset_playback();
        }
        if input.toggle_hurtboxes {
            self.show_hurtboxes = !self.show_hurtboxes;
        }
        if input.toggle_hitboxes {
            self.show_hitboxes = !self.show_hitboxes;
        }
        if input.toggle_pivot {
            self.show_pivot = !self.show_pivot;
        }
        if input.toggle_dummy {
            self.show_dummy = !self.show_dummy;
        }
        if input.pause_toggle {
            self.paused = !self.paused;
        }

        if !self.paused || input.step_frame {
            self.advance_frame();
        }
    }

    /// Returns the selected character.
    pub const fn character(&self) -> CharacterId {
        self.character
    }

    /// Returns the selected move.
    pub const fn selected_move(&self) -> CombatLabMove {
        self.selected_move
    }

    /// Returns the selected lab pose or move playback mode.
    pub const fn pose(&self) -> CombatLabPose {
        self.pose
    }

    /// Returns the isolated fighter.
    pub const fn fighter(&self) -> &Fighter {
        &self.fighter
    }

    /// Returns active lab projectiles.
    pub fn projectiles(&self) -> &[Projectile] {
        &self.projectiles
    }

    /// Returns elapsed playback frames.
    pub const fn current_frame(&self) -> FrameCount {
        self.current_frame
    }

    /// Returns elapsed playback time for sprite selection.
    pub fn elapsed_seconds(&self) -> f32 {
        self.current_frame.as_seconds()
    }

    /// Returns whether playback is paused.
    pub const fn paused(&self) -> bool {
        self.paused
    }

    /// Returns whether hurtboxes should be drawn.
    pub const fn show_hurtboxes(&self) -> bool {
        self.show_hurtboxes
    }

    /// Returns whether hitboxes should be drawn.
    pub const fn show_hitboxes(&self) -> bool {
        self.show_hitboxes
    }

    /// Returns whether pivot axes should be drawn.
    pub const fn show_pivot(&self) -> bool {
        self.show_pivot
    }

    /// Returns whether the optional contact dummy should be drawn.
    pub const fn show_dummy(&self) -> bool {
        self.show_dummy
    }

    /// Returns estimated advantage and spacing for the selected move.
    pub fn advantage(&self) -> Option<CombatLabAdvantage> {
        if !self.pose.is_move_playback() {
            return None;
        }

        let fighter = fighter_for(self.character);
        analyze_advantage(self.character, &fighter, self.selected_move.analysis_move())
    }

    /// Returns the dummy body positioned at the selected move contact point.
    pub fn dummy_body_rect(&self) -> Rect {
        if !self.pose.is_move_playback() {
            return default_dummy_body();
        }

        let fighter = fighter_for(self.character);
        contact_dummy_body(self.character, &fighter, self.selected_move.analysis_move())
            .unwrap_or_else(default_dummy_body)
    }

    fn advance_frame(&mut self) {
        if !self.pose.is_move_playback() {
            self.current_frame = FrameCount::new(self.current_frame.get().saturating_add(1));
            return;
        }

        if self.current_frame == FrameCount::ZERO && self.selected_move == CombatLabMove::Projectile
        {
            self.spawn_projectile();
        }

        let input = if self.current_frame == FrameCount::ZERO {
            self.selected_move.first_frame_input()
        } else {
            FighterInput::default()
        };

        self.fighter.update(FIXED_TIMESTEP, input);
        for projectile in &mut self.projectiles {
            projectile.update(FIXED_TIMESTEP);
        }
        self.projectiles.retain(|projectile| {
            projectile.rect().right() >= 0.0 && projectile.rect().x <= WINDOW_WIDTH as f32
        });
        self.current_frame = FrameCount::new(self.current_frame.get().saturating_add(1));
    }

    fn spawn_projectile(&mut self) {
        if !self.fighter.can_fire_projectile() {
            return;
        }

        self.projectiles
            .push(Projectile::from_fighter(&self.fighter));
        self.fighter.mark_projectile_fired();
    }

    fn reset_playback(&mut self) {
        self.fighter = fighter_for(self.character);
        self.projectiles.clear();
        self.current_frame = FrameCount::ZERO;
        apply_pose_to_fighter(&mut self.fighter, self.pose);
    }

    fn select_next_move(&mut self) {
        let index = move_index(self.selected_move);
        self.selected_move = CombatLabMove::ALL[(index + 1) % CombatLabMove::ALL.len()];
        self.reset_playback();
    }

    fn select_previous_move(&mut self) {
        let index = move_index(self.selected_move);
        let next = if index == 0 {
            CombatLabMove::ALL.len() - 1
        } else {
            index - 1
        };
        self.selected_move = CombatLabMove::ALL[next];
        self.reset_playback();
    }

    fn select_next_pose(&mut self) {
        let index = pose_index(self.pose);
        self.pose = CombatLabPose::ALL[(index + 1) % CombatLabPose::ALL.len()];
        self.reset_playback();
    }

    fn select_previous_pose(&mut self) {
        let index = pose_index(self.pose);
        let next = if index == 0 {
            CombatLabPose::ALL.len() - 1
        } else {
            index - 1
        };
        self.pose = CombatLabPose::ALL[next];
        self.reset_playback();
    }
}

fn apply_pose_to_fighter(fighter: &mut Fighter, pose: CombatLabPose) {
    if matches!(
        pose,
        CombatLabPose::Move | CombatLabPose::Idle | CombatLabPose::Hit | CombatLabPose::Victory
    ) {
        return;
    }

    match pose {
        CombatLabPose::Crouch => {
            fighter.crouching = true;
        }
        CombatLabPose::Jump => {
            fighter.grounded = false;
            fighter.position.y -= LAB_JUMP_PREVIEW_HEIGHT;
        }
        CombatLabPose::Block => {
            fighter.blocking = true;
        }
        _ => {}
    }
}

fn fighter_for(character: CharacterId) -> Fighter {
    let spec = character_spec(character);
    Fighter::new_with_loadout(
        slot_for(character),
        spec.fighter_name,
        spec.stats.max_health,
        spec.move_ids,
        LAB_FIGHTER_X,
    )
}

fn slot_for(character: CharacterId) -> PlayerSlot {
    match character {
        CharacterId::Rust => PlayerSlot::One,
        CharacterId::Duke => PlayerSlot::Two,
    }
}

fn move_index(selected_move: CombatLabMove) -> usize {
    CombatLabMove::ALL
        .iter()
        .position(|candidate| *candidate == selected_move)
        .unwrap_or(0)
}

fn pose_index(pose: CombatLabPose) -> usize {
    CombatLabPose::ALL
        .iter()
        .position(|candidate| *candidate == pose)
        .unwrap_or(0)
}
