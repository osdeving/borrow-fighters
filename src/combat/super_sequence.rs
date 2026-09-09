//! Defines the four authored super schedules and their presentation snapshots.
//!
//! System: Combat data. A World-owned sequence confirms capture, advances these
//! fixed phases and applies only listed contacts; decorations never deal damage.

use super::{
    fighter::{Facing, PlayerSlot},
    move_data::MoveId,
};
use crate::{characters::CharacterId, math::vec2::Vec2};

pub const SUPER_FREEZE_FRAMES: u32 = 8;
pub const SUPER_TARGET_LAND_END: u32 = 20;
pub const DUKE_CLONE_COUNT: u32 = 12;
pub const DUKE_COLLECT_START: u32 = 84;
pub const DUKE_CLONE_CADENCE: u32 = 12;
pub const CPP_FOOTSHOT_TICK: u32 = 44;
pub const CPP_BARRAGE_START: u32 = 204;
pub const CPP_BARRAGE_CADENCE: u32 = 10;
pub const CPP_BARRAGE_HITS: u32 = 8;

/// Authored phases, shared by gameplay, artwork and audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuperPhase {
    Freeze,
    RustBlackout,
    RustBuild,
    RustCharge,
    RustPulse,
    DukeTrashFall,
    DukeTrashSettle,
    DukeCollect,
    DukeGiantDrop,
    DukeImpact,
    CTerminalStorm,
    CBlueScreen,
    CBios,
    CReboot,
    CppFootshot,
    CppHop,
    CppRage,
    CppCharge,
    CppBarrage,
    CppFinisher,
    Restore,
}

/// Half-open frame interval, with exactly one phase active at each sequence tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuperPhaseSpan {
    pub phase: SuperPhase,
    pub start: u32,
    pub end: u32,
}

/// One authoritative damage event; even distant targets are already captured.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuperContact {
    pub tick: u32,
    pub damage: i32,
    pub knockdown: bool,
}

/// Immutable character-specific sequence tuning.
#[derive(Clone, Copy, Debug)]
pub struct SuperSpec {
    pub character: CharacterId,
    pub move_id: MoveId,
    pub label: &'static str,
    pub duration_frames: u32,
    pub phases: &'static [SuperPhaseSpan],
    pub contacts: &'static [SuperContact],
}

const fn phase(phase: SuperPhase, start: u32, end: u32) -> SuperPhaseSpan {
    SuperPhaseSpan { phase, start, end }
}

use SuperPhase::*;

const RUST_PHASES: &[SuperPhaseSpan] = &[
    phase(Freeze, 0, 8),
    phase(RustBlackout, 8, 11),
    phase(RustBuild, 11, 125),
    phase(RustCharge, 125, 205),
    phase(RustPulse, 205, 235),
    phase(Restore, 235, 300),
];

const DUKE_PHASES: &[SuperPhaseSpan] = &[
    phase(Freeze, 0, 8),
    phase(DukeTrashFall, 8, 60),
    phase(DukeTrashSettle, 60, 84),
    phase(DukeCollect, 84, 228),
    phase(DukeGiantDrop, 228, 264),
    phase(DukeImpact, 264, 302),
    phase(Restore, 302, 350),
];

const C_PHASES: &[SuperPhaseSpan] = &[
    phase(Freeze, 0, 8),
    phase(CTerminalStorm, 8, 105),
    phase(CBlueScreen, 105, 180),
    phase(CBios, 180, 222),
    phase(CReboot, 222, 260),
    phase(Restore, 260, 320),
];

const CPP_PHASES: &[SuperPhaseSpan] = &[
    phase(Freeze, 0, 8),
    phase(CppFootshot, 8, 62),
    phase(CppHop, 62, 110),
    phase(CppRage, 110, 140),
    phase(CppCharge, 140, 204),
    phase(CppBarrage, 204, 284),
    phase(CppFinisher, 284, 312),
    phase(Restore, 312, 356),
];

/// Returns authored data, or None for the unchanged Go/Python local cinematographics.
pub const fn super_spec(character: CharacterId) -> Option<SuperSpec> {
    Some(match character {
        CharacterId::Rust => SuperSpec {
            character,
            move_id: MoveId::RustOwnershipEclipse,
            label: "Ownership Eclipse",
            duration_frames: 300,
            phases: RUST_PHASES,
            contacts: &[SuperContact {
                tick: 212,
                damage: 28,
                knockdown: true,
            }],
        },
        CharacterId::Duke => SuperSpec {
            character,
            move_id: MoveId::DukeJvmOverdrive,
            label: "Garbage Collector",
            duration_frames: 350,
            phases: DUKE_PHASES,
            contacts: &[SuperContact {
                tick: 264,
                damage: 32,
                knockdown: true,
            }],
        },
        CharacterId::C => SuperSpec {
            character,
            move_id: MoveId::CKernelPanic,
            label: "General Protection Fault / #GP",
            duration_frames: 320,
            phases: C_PHASES,
            contacts: &[SuperContact {
                tick: 222,
                damage: 32,
                knockdown: true,
            }],
        },
        CharacterId::Cpp => SuperSpec {
            character,
            move_id: MoveId::CppTemplateSingularity,
            label: "Undefined Behavior: Footgun",
            duration_frames: 356,
            phases: CPP_PHASES,
            contacts: &[
                SuperContact {
                    tick: 204,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 214,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 224,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 234,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 244,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 254,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 264,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 274,
                    damage: 3,
                    knockdown: false,
                },
                SuperContact {
                    tick: 284,
                    damage: 6,
                    knockdown: true,
                },
            ],
        },
        CharacterId::Go | CharacterId::Python => return None,
    })
}

/// Captured actors and the live authorial clock. Anchors use feet-center coordinates.
#[derive(Clone, Debug)]
pub struct SuperSequence {
    pub character: CharacterId,
    pub move_id: MoveId,
    pub label: &'static str,
    pub attacker: PlayerSlot,
    pub target: PlayerSlot,
    pub guarded: bool,
    pub target_crouching: bool,
    pub tick: u32,
    pub duration_frames: u32,
    pub attacker_origin: Vec2,
    pub target_origin: Vec2,
    pub attacker_anchor: Vec2,
    pub target_anchor: Vec2,
    pub facing: Facing,
    pub(crate) fractional_ticks: f32,
    pub(crate) charge_destination_x: f32,
}

impl SuperSequence {
    /// The single active phase, including restoration on the last tick.
    pub fn phase(&self) -> SuperPhase {
        self.phase_span().phase
    }

    /// Returns authoritative boundaries for the current phase.
    pub fn phase_span(&self) -> SuperPhaseSpan {
        let spec = super_spec(self.character).expect("authored super has a spec");
        spec.phases
            .iter()
            .copied()
            .find(|span| self.tick >= span.start && self.tick < span.end)
            .unwrap_or(*spec.phases.last().expect("super has restoration"))
    }

    /// Elapsed phase time, clamped for snapshots taken at sequence completion.
    pub fn phase_progress(&self) -> f32 {
        let span = self.phase_span();
        (self.tick.saturating_sub(span.start) as f32 / (span.end - span.start) as f32)
            .clamp(0.0, 1.0)
    }

    /// Overall normalized sequence time.
    pub fn progress(&self) -> f32 {
        (self.tick as f32 / self.duration_frames as f32).clamp(0.0, 1.0)
    }
}
