//! Defines data-driven audio events and manifest routing.
//!
//! System: Audio domain. This module is renderer-free and Raylib-free; it
//! names gameplay audio events and resolves them against manifest bindings.

use std::{fs, path::Path};

use serde::Deserialize;

use crate::{
    characters::CharacterId,
    combat::{fighter::PlayerSlot, move_data::MoveId},
};

/// Gameplay cue emitted by systems that want audio feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioCue {
    UiNavigate,
    UiConfirm,
    UiBack,
    MatchStart,
    MatchCountdownEleven,
    MatchCountdownTen,
    MatchCountdownOne,
    MatchCountdownFight,
    MatchVictory,
    FighterAttackStart,
    FighterAttackWhiff,
    FighterProjectileCast,
    FighterHurt,
    FighterBlock,
    CombatHit,
    CombatBlock,
    ProjectileImpact,
}

impl AudioCue {
    /// Returns the stable key used in audio manifests.
    pub const fn key(self) -> &'static str {
        match self {
            Self::UiNavigate => "ui.navigate",
            Self::UiConfirm => "ui.confirm",
            Self::UiBack => "ui.back",
            Self::MatchStart => "match.start",
            Self::MatchCountdownEleven => "match.countdown.11",
            Self::MatchCountdownTen => "match.countdown.10",
            Self::MatchCountdownOne => "match.countdown.01",
            Self::MatchCountdownFight => "match.countdown.fight",
            Self::MatchVictory => "match.victory",
            Self::FighterAttackStart => "fighter.attack.start",
            Self::FighterAttackWhiff => "fighter.attack.whiff",
            Self::FighterProjectileCast => "fighter.projectile.cast",
            Self::FighterHurt => "fighter.hurt",
            Self::FighterBlock => "fighter.block",
            Self::CombatHit => "combat.hit",
            Self::CombatBlock => "combat.block",
            Self::ProjectileImpact => "projectile.impact",
        }
    }

    /// Parses a stable manifest key.
    pub fn from_key(value: &str) -> Option<Self> {
        match value {
            "ui.navigate" => Some(Self::UiNavigate),
            "ui.confirm" => Some(Self::UiConfirm),
            "ui.back" => Some(Self::UiBack),
            "match.start" => Some(Self::MatchStart),
            "match.countdown.11" => Some(Self::MatchCountdownEleven),
            "match.countdown.10" => Some(Self::MatchCountdownTen),
            "match.countdown.01" => Some(Self::MatchCountdownOne),
            "match.countdown.fight" => Some(Self::MatchCountdownFight),
            "match.victory" => Some(Self::MatchVictory),
            "fighter.attack.start" => Some(Self::FighterAttackStart),
            "fighter.attack.whiff" => Some(Self::FighterAttackWhiff),
            "fighter.projectile.cast" => Some(Self::FighterProjectileCast),
            "fighter.hurt" => Some(Self::FighterHurt),
            "fighter.block" => Some(Self::FighterBlock),
            "combat.hit" => Some(Self::CombatHit),
            "combat.block" => Some(Self::CombatBlock),
            "projectile.impact" => Some(Self::ProjectileImpact),
            _ => None,
        }
    }
}

/// Context attached to a gameplay audio event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioEvent {
    pub cue: AudioCue,
    pub slot: Option<PlayerSlot>,
    pub character: Option<CharacterId>,
    pub move_id: Option<MoveId>,
    pub environment: Option<&'static str>,
}

impl AudioEvent {
    /// Creates an event without extra routing context.
    pub const fn new(cue: AudioCue) -> Self {
        Self {
            cue,
            slot: None,
            character: None,
            move_id: None,
            environment: None,
        }
    }

    /// Menu cursor movement cue.
    pub const fn ui_navigate() -> Self {
        Self::new(AudioCue::UiNavigate)
    }

    /// Menu confirm or toggle cue.
    pub const fn ui_confirm() -> Self {
        Self::new(AudioCue::UiConfirm)
    }

    /// Menu back or return cue.
    pub const fn ui_back() -> Self {
        Self::new(AudioCue::UiBack)
    }

    /// Match intro or round-start cue.
    pub const fn match_start() -> Self {
        Self::new(AudioCue::MatchStart)
    }

    /// First pre-fight countdown cue. Visual text is `11`, audio is "three".
    pub const fn match_countdown_eleven() -> Self {
        Self::new(AudioCue::MatchCountdownEleven)
    }

    /// Second pre-fight countdown cue. Visual text is `10`, audio is "two".
    pub const fn match_countdown_ten() -> Self {
        Self::new(AudioCue::MatchCountdownTen)
    }

    /// Third pre-fight countdown cue. Visual text is `01`, audio is "one".
    pub const fn match_countdown_one() -> Self {
        Self::new(AudioCue::MatchCountdownOne)
    }

    /// Final pre-fight cue.
    pub const fn match_countdown_fight() -> Self {
        Self::new(AudioCue::MatchCountdownFight)
    }

    /// Match victory cue for one character.
    pub const fn match_victory(slot: PlayerSlot, character: CharacterId) -> Self {
        Self::new(AudioCue::MatchVictory).with_fighter(slot, character)
    }

    /// Close attack startup cue.
    pub const fn fighter_attack_start(
        slot: PlayerSlot,
        character: CharacterId,
        move_id: MoveId,
    ) -> Self {
        Self::new(AudioCue::FighterAttackStart)
            .with_fighter(slot, character)
            .with_move(move_id)
    }

    /// Close attack whiff cue.
    pub const fn fighter_attack_whiff(
        slot: PlayerSlot,
        character: CharacterId,
        move_id: MoveId,
    ) -> Self {
        Self::new(AudioCue::FighterAttackWhiff)
            .with_fighter(slot, character)
            .with_move(move_id)
    }

    /// Projectile cast cue.
    pub const fn fighter_projectile_cast(slot: PlayerSlot, character: CharacterId) -> Self {
        Self::new(AudioCue::FighterProjectileCast).with_fighter(slot, character)
    }

    /// Hurt voice cue for a defender.
    pub const fn fighter_hurt(slot: PlayerSlot, character: CharacterId) -> Self {
        Self::new(AudioCue::FighterHurt).with_fighter(slot, character)
    }

    /// Guard voice or block effort cue for a defender.
    pub const fn fighter_block(slot: PlayerSlot, character: CharacterId) -> Self {
        Self::new(AudioCue::FighterBlock).with_fighter(slot, character)
    }

    /// Physical hit impact cue.
    pub const fn combat_hit(slot: PlayerSlot, character: CharacterId, move_id: MoveId) -> Self {
        Self::new(AudioCue::CombatHit)
            .with_fighter(slot, character)
            .with_move(move_id)
    }

    /// Physical guard impact cue.
    pub const fn combat_block(slot: PlayerSlot, character: CharacterId, move_id: MoveId) -> Self {
        Self::new(AudioCue::CombatBlock)
            .with_fighter(slot, character)
            .with_move(move_id)
    }

    /// Projectile impact cue.
    pub const fn projectile_impact(slot: PlayerSlot, character: CharacterId) -> Self {
        Self::new(AudioCue::ProjectileImpact).with_fighter(slot, character)
    }

    /// Attaches fighter identity.
    pub const fn with_fighter(mut self, slot: PlayerSlot, character: CharacterId) -> Self {
        self.slot = Some(slot);
        self.character = Some(character);
        self
    }

    /// Attaches move identity.
    pub const fn with_move(mut self, move_id: MoveId) -> Self {
        self.move_id = Some(move_id);
        self
    }
}

/// Background music tracks known by the runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MusicTrack {
    Menu,
    Combat,
    CombatChiptuneBattle,
    CombatRinsTheme,
    CombatEightBitBattle,
    CombatConsoleFloor,
    CombatRandomEncounter,
    CombatDeterminedPursuit,
}

impl MusicTrack {
    /// Returns the stable key used in audio manifests.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Menu => "menu",
            Self::Combat => "combat",
            Self::CombatChiptuneBattle => "combat-chiptune-battle",
            Self::CombatRinsTheme => "combat-rins-theme",
            Self::CombatEightBitBattle => "combat-8bit-battle",
            Self::CombatConsoleFloor => "combat-console-floor",
            Self::CombatRandomEncounter => "combat-random-encounter",
            Self::CombatDeterminedPursuit => "combat-determined-pursuit",
        }
    }

    /// Parses a stable manifest key.
    pub fn from_key(value: &str) -> Option<Self> {
        match value {
            "menu" => Some(Self::Menu),
            "combat" => Some(Self::Combat),
            "combat-chiptune-battle" => Some(Self::CombatChiptuneBattle),
            "combat-rins-theme" => Some(Self::CombatRinsTheme),
            "combat-8bit-battle" => Some(Self::CombatEightBitBattle),
            "combat-console-floor" => Some(Self::CombatConsoleFloor),
            "combat-random-encounter" => Some(Self::CombatRandomEncounter),
            "combat-determined-pursuit" => Some(Self::CombatDeterminedPursuit),
            _ => None,
        }
    }
}

/// Parsed audio manifest.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AudioManifest {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub clips: Vec<AudioClipDefinition>,
    #[serde(default)]
    pub music: Vec<AudioMusicDefinition>,
    #[serde(default)]
    pub bindings: Vec<AudioBindingDefinition>,
}

impl AudioManifest {
    /// Loads an audio manifest from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, AudioManifestError> {
        let path = path.as_ref();
        let text = fs::read_to_string(path).map_err(|source| AudioManifestError::Read {
            path: path.display().to_string(),
            source,
        })?;
        serde_json::from_str(&text).map_err(|source| AudioManifestError::Parse {
            path: path.display().to_string(),
            source,
        })
    }

    /// Parses an audio manifest from JSON text.
    pub fn from_json(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
}

/// One sound asset entry.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AudioClipDefinition {
    pub id: String,
    pub file: String,
    #[serde(default = "default_bus")]
    pub bus: String,
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default = "default_pitch")]
    pub pitch: f32,
    #[serde(default = "default_pan")]
    pub pan: f32,
    #[serde(default)]
    pub required: bool,
}

/// One streamed background music entry.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AudioMusicDefinition {
    pub id: String,
    pub file: String,
    #[serde(default = "default_music_volume")]
    pub volume: f32,
    #[serde(default = "default_pitch")]
    pub pitch: f32,
    #[serde(default = "default_true")]
    pub looping: bool,
    #[serde(default)]
    pub required: bool,
}

/// Event-to-clip routing entry.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AudioBindingDefinition {
    pub cue: String,
    #[serde(default)]
    pub character: Option<String>,
    #[serde(rename = "move", default)]
    pub move_key: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub clips: Vec<String>,
}

impl AudioBindingDefinition {
    fn specificity(&self) -> usize {
        usize::from(self.character.is_some())
            + usize::from(self.move_key.is_some())
            + usize::from(self.environment.is_some())
    }
}

/// Prepared manifest with matching helpers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AudioBank {
    manifest: AudioManifest,
}

impl AudioBank {
    /// Creates a bank from a parsed manifest.
    pub const fn new(manifest: AudioManifest) -> Self {
        Self { manifest }
    }

    /// Loads a bank from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, AudioManifestError> {
        AudioManifest::load(path).map(Self::new)
    }

    /// Returns clip definitions.
    pub fn clips(&self) -> &[AudioClipDefinition] {
        &self.manifest.clips
    }

    /// Returns streamed music definitions.
    pub fn music_tracks(&self) -> &[AudioMusicDefinition] {
        &self.manifest.music
    }

    /// Returns binding definitions.
    pub fn bindings(&self) -> &[AudioBindingDefinition] {
        &self.manifest.bindings
    }

    /// Finds the most specific binding for an event.
    pub fn binding_for_event(
        &self,
        event: &AudioEvent,
    ) -> Option<(usize, &AudioBindingDefinition)> {
        let index = self.binding_index_for_event(event)?;
        self.manifest
            .bindings
            .get(index)
            .map(|binding| (index, binding))
    }

    /// Finds the most specific binding index for an event.
    pub fn binding_index_for_event(&self, event: &AudioEvent) -> Option<usize> {
        self.manifest
            .bindings
            .iter()
            .enumerate()
            .filter(|(_, binding)| binding_matches_event(binding, event))
            .max_by_key(|(_, binding)| binding.specificity())
            .map(|(index, _)| index)
    }

    /// Returns clip ids for a binding index.
    pub fn binding_clip_ids(&self, binding_index: usize) -> Option<&[String]> {
        self.manifest
            .bindings
            .get(binding_index)
            .map(|binding| binding.clips.as_slice())
    }
}

/// Manifest loading error.
#[derive(Debug)]
pub enum AudioManifestError {
    Read {
        path: String,
        source: std::io::Error,
    },
    Parse {
        path: String,
        source: serde_json::Error,
    },
}

impl std::fmt::Display for AudioManifestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "could not read audio manifest {path}: {source}")
            }
            Self::Parse { path, source } => {
                write!(formatter, "could not parse audio manifest {path}: {source}")
            }
        }
    }
}

impl std::error::Error for AudioManifestError {}

fn binding_matches_event(binding: &AudioBindingDefinition, event: &AudioEvent) -> bool {
    if AudioCue::from_key(&binding.cue) != Some(event.cue) {
        return false;
    }

    if let Some(character) = &binding.character
        && event.character.map(CharacterId::audio_key) != Some(character.as_str())
    {
        return false;
    }

    if let Some(move_key) = &binding.move_key
        && event.move_id.map(MoveId::audio_key) != Some(move_key.as_str())
    {
        return false;
    }

    if let Some(environment) = &binding.environment
        && event.environment != Some(environment.as_str())
    {
        return false;
    }

    true
}

fn default_bus() -> String {
    "sfx".to_owned()
}

const fn default_volume() -> f32 {
    1.0
}

const fn default_pitch() -> f32 {
    1.0
}

const fn default_pan() -> f32 {
    0.5
}

const fn default_music_volume() -> f32 {
    0.45
}

const fn default_true() -> bool {
    true
}
