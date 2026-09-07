//! Owns autoplay state for the single-character move showcase.
//!
//! System: Training scene. This module sequences existing Combat Lab playback
//! without owning combat rules, rendering, or sprite selection.

use crate::characters::CharacterId;
use crate::engine::sprites::SpriteManifest;

use super::combat_lab::{CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions};

const REST_FRAMES: u32 = 16;
const DEFAULT_MOVE_FRAMES: u32 = 64;
const AIR_MOVE_FRAMES: u32 = 72;
const PROJECTILE_MOVE_FRAMES: u32 = 96;

/// Startup options for the autoplay move showcase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MoveShowcaseOptions {
    pub character: CharacterId,
}

/// Autoplaying single-character showcase for all current combat verbs.
#[derive(Clone, Debug)]
pub struct MoveShowcase {
    character: CharacterId,
    move_index: usize,
    lab: CombatLab,
    combat_manifest: Option<SpriteManifest>,
    rest_frames_remaining: u32,
    paused: bool,
}

impl Default for MoveShowcase {
    fn default() -> Self {
        Self::new(MoveShowcaseOptions::default())
    }
}

impl MoveShowcase {
    /// Creates a showcase for one character, starting at the first move.
    pub fn new(options: MoveShowcaseOptions) -> Self {
        Self {
            character: options.character,
            move_index: 0,
            lab: lab_for(options.character, CombatLabMove::ALL[0]),
            combat_manifest: None,
            rest_frames_remaining: 0,
            paused: false,
        }
    }

    /// Handles one fixed showcase tick.
    pub fn update(&mut self, input: CombatLabInput) {
        if input.previous_move {
            self.select_previous_move();
        }
        if input.next_move {
            self.select_next_move();
        }
        if input.replay || input.reset {
            self.reset_current_move();
        }
        if input.pause_toggle {
            self.paused = !self.paused;
        }

        if self.paused {
            return;
        }

        if self.rest_frames_remaining > 0 {
            self.rest_frames_remaining -= 1;
            return;
        }

        self.lab.update(CombatLabInput::default());

        if u32::from(self.lab.current_frame().get()) >= showcase_frames_for(self.selected_move()) {
            self.select_next_move_with_rest();
        }
    }

    /// Returns the selected showcase character.
    pub const fn character(&self) -> CharacterId {
        self.character
    }

    /// Returns the current move.
    pub fn selected_move(&self) -> CombatLabMove {
        CombatLabMove::ALL[self.move_index]
    }

    /// Returns the 1-based move number for display.
    pub const fn move_number(&self) -> usize {
        self.move_index + 1
    }

    /// Returns the total number of moves in the showcase.
    pub const fn move_count(&self) -> usize {
        CombatLabMove::ALL.len()
    }

    /// Returns the isolated lab snapshot used for rendering.
    pub const fn lab(&self) -> &CombatLab {
        &self.lab
    }

    /// Keeps baseline metadata attached when autoplay creates the next lab snapshot.
    pub fn set_combat_manifest(&mut self, manifest: Option<SpriteManifest>) {
        self.lab.set_combat_manifest(manifest.clone());
        self.combat_manifest = manifest;
    }

    /// Returns whether autoplay is paused.
    pub const fn paused(&self) -> bool {
        self.paused
    }

    /// Returns whether the scene is between two moves.
    pub const fn resting(&self) -> bool {
        self.rest_frames_remaining > 0
    }

    fn select_next_move(&mut self) {
        self.move_index = (self.move_index + 1) % CombatLabMove::ALL.len();
        self.reset_current_move();
    }

    fn select_previous_move(&mut self) {
        self.move_index = if self.move_index == 0 {
            CombatLabMove::ALL.len() - 1
        } else {
            self.move_index - 1
        };
        self.reset_current_move();
    }

    fn select_next_move_with_rest(&mut self) {
        self.move_index = (self.move_index + 1) % CombatLabMove::ALL.len();
        self.lab = lab_for(self.character, self.selected_move());
        self.lab.set_combat_manifest(self.combat_manifest.clone());
        self.rest_frames_remaining = REST_FRAMES;
    }

    fn reset_current_move(&mut self) {
        self.lab = lab_for(self.character, self.selected_move());
        self.lab.set_combat_manifest(self.combat_manifest.clone());
        self.rest_frames_remaining = 0;
    }
}

const fn showcase_frames_for(selected_move: CombatLabMove) -> u32 {
    match selected_move {
        CombatLabMove::AirPunch | CombatLabMove::AirKick => AIR_MOVE_FRAMES,
        CombatLabMove::Projectile => PROJECTILE_MOVE_FRAMES,
        _ => DEFAULT_MOVE_FRAMES,
    }
}

fn lab_for(character: CharacterId, selected_move: CombatLabMove) -> CombatLab {
    CombatLab::new(CombatLabOptions {
        character,
        selected_move,
        ..CombatLabOptions::default()
    })
}
