//! Owns the two selection cursors, confirmations, random picks and match setup.
//!
//! Input and drawing share this small state without depending on Raylib. Future
//! slots are visible but never resolve to fighters or enter random selection.

use crate::characters::CharacterId;
use crate::game::arena::ArenaId;
use crate::scenes::preferences::PlayMode;

pub const PUBLIC_ROSTER: [CharacterId; 5] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];
pub const SLOT_COUNT: usize = 8;
pub const RANDOM_SLOT: usize = 5;

/// One owner's navigation edges; holding a button cannot confirm a second owner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SelectCommand {
    pub horizontal: i8,
    pub vertical: i8,
    pub confirm: bool,
    pub back: bool,
}

/// Screen commands translated at the platform boundary.
#[derive(Clone, Copy, Debug, Default)]
pub struct SelectInput {
    pub players: [SelectCommand; 2],
    pub shared: SelectCommand,
    pub hover: Option<usize>,
    pub click: bool,
    pub owner: Option<usize>,
    pub cycle_mode: bool,
    pub arena_direction: i8,
    pub launch: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectAction {
    #[default]
    Stay,
    Move,
    Confirm,
    Reject,
    Back,
    Launch,
}

/// Current selection, including the stable resolution of an accepted random pick.
#[derive(Clone, Debug)]
pub struct CharacterSelect {
    cursors: [usize; 2],
    resolved: [CharacterId; 2],
    ready: [bool; 2],
    active: usize,
    pub mode: PlayMode,
    pub arena: ArenaId,
    seed: u64,
    pub elapsed: f32,
    pub focus_elapsed: [f32; 2],
    accepting_input: bool,
}

impl CharacterSelect {
    pub fn new(characters: [CharacterId; 2], arena: ArenaId, mode: PlayMode, seed: u64) -> Self {
        let cursors = characters.map(|id| PUBLIC_ROSTER.iter().position(|&v| v == id).unwrap_or(0));
        Self {
            cursors,
            resolved: cursors.map(|i| PUBLIC_ROSTER[i]),
            ready: [false; 2],
            active: 0,
            mode,
            arena,
            seed,
            elapsed: 0.0,
            focus_elapsed: [0.0; 2],
            accepting_input: false,
        }
    }

    pub fn cursor(&self, owner: usize) -> usize {
        self.cursors[owner]
    }
    pub fn ready(&self, owner: usize) -> bool {
        self.ready[owner]
    }
    pub fn active(&self) -> usize {
        self.active
    }
    pub fn both_ready(&self) -> bool {
        self.ready.iter().all(|&ready| ready)
    }
    pub fn characters(&self) -> [CharacterId; 2] {
        self.resolved
    }

    pub fn preview(&self, owner: usize) -> CharacterId {
        if self.ready[owner] {
            return self.resolved[owner];
        }
        PUBLIC_ROSTER
            .get(self.cursors[owner])
            .copied()
            .unwrap_or_else(|| {
                if self.cursors[owner] == RANDOM_SLOT {
                    PUBLIC_ROSTER[((self.elapsed * 5.0) as usize + owner * 2) % PUBLIC_ROSTER.len()]
                } else {
                    self.resolved[owner]
                }
            })
    }

    pub fn update(&mut self, dt: f32, input: SelectInput) -> SelectAction {
        self.elapsed += dt.max(0.0);
        for t in &mut self.focus_elapsed {
            *t += dt.max(0.0);
        }
        if !self.accepting_input {
            self.accepting_input = true;
            return SelectAction::Stay;
        }
        if input.cycle_mode {
            self.mode = match self.mode {
                PlayMode::AgainstCpu => PlayMode::LocalDuel,
                PlayMode::LocalDuel => PlayMode::WatchDemo,
                PlayMode::WatchDemo => PlayMode::AgainstCpu,
            };
            self.ready = [false; 2];
            self.active = 0;
            return SelectAction::Move;
        }
        if input.arena_direction != 0 {
            self.arena = if input.arena_direction > 0 {
                self.arena.next()
            } else {
                self.arena.previous()
            };
            return SelectAction::Move;
        }
        if let Some(owner) = input.owner.filter(|&i| i < 2) {
            self.active = owner;
        }
        let confirm = input.shared.confirm || input.players.iter().any(|command| command.confirm);
        let cancelling = input.shared.back || input.players.iter().any(|command| command.back);
        if self.both_ready() && !cancelling && (input.launch || confirm) {
            return SelectAction::Launch;
        }
        let mut action = SelectAction::Stay;
        let shared_owner = self.active;
        if let Some(index) = input.hover.filter(|&i| i < SLOT_COUNT) {
            if !self.ready[shared_owner] && self.cursors[shared_owner] != index {
                self.cursors[shared_owner] = index;
                self.focus_elapsed[shared_owner] = 0.0;
                action = SelectAction::Move;
            }
        }
        let mut shared = input.shared;
        shared.confirm |= input.click && input.hover.is_some();
        let next = self.command(shared_owner, shared);
        if next != SelectAction::Stay {
            action = next;
        }
        if matches!(action, SelectAction::Back | SelectAction::Launch) {
            return action;
        }
        for (owner, command) in input.players.into_iter().enumerate() {
            // A local controller can undo its own pick, not the other player's.
            // Esc, mouse and the shared single-player command can still go back.
            if command.back && !self.ready[owner] && self.ready[1 - owner] {
                continue;
            }
            let next = self.command(owner, command);
            if next != SelectAction::Stay {
                action = next;
            }
            if matches!(action, SelectAction::Back | SelectAction::Launch) {
                return action;
            }
        }
        action
    }

    fn command(&mut self, owner: usize, command: SelectCommand) -> SelectAction {
        if command.back {
            if self.ready[owner] {
                self.ready[owner] = false;
                self.active = owner;
            } else if self.ready.iter().any(|&v| v) {
                self.ready = [false; 2];
                self.active = 0;
            } else {
                return SelectAction::Back;
            }
            return SelectAction::Move;
        }
        if self.ready[owner] {
            return SelectAction::Stay;
        }
        let dx = command.horizontal.signum() as isize;
        let dy = command.vertical.signum() as isize;
        if dx != 0 || dy != 0 {
            let cursor = self.cursors[owner];
            let col = (cursor as isize % 4 + dx).rem_euclid(4) as usize;
            let row = (cursor as isize / 4 + dy).rem_euclid(2) as usize;
            self.cursors[owner] = row * 4 + col;
            self.active = owner;
            self.focus_elapsed[owner] = 0.0;
        }
        if command.confirm {
            if self.cursors[owner] > RANDOM_SLOT {
                return SelectAction::Reject;
            }
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.resolved[owner] = PUBLIC_ROSTER
                .get(self.cursors[owner])
                .copied()
                .unwrap_or(PUBLIC_ROSTER[(self.seed >> 32) as usize % PUBLIC_ROSTER.len()]);
            self.ready[owner] = true;
            self.focus_elapsed[owner] = 0.0;
            self.active = if self.ready[1 - owner] {
                owner
            } else {
                1 - owner
            };
            return SelectAction::Confirm;
        }
        if dx != 0 || dy != 0 {
            SelectAction::Move
        } else {
            SelectAction::Stay
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn screen() -> CharacterSelect {
        let mut screen = CharacterSelect::new(
            [CharacterId::Rust, CharacterId::Duke],
            ArenaId::Sirius,
            PlayMode::AgainstCpu,
            42,
        );
        screen.update(0.0, SelectInput::default());
        screen
    }
    #[test]
    fn one_confirmation_never_confirms_both_sides() {
        let mut s = screen();
        assert_eq!(
            s.update(
                0.0,
                SelectInput {
                    shared: SelectCommand {
                        confirm: true,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ),
            SelectAction::Confirm
        );
        assert!(s.ready(0));
        assert!(!s.ready(1));
        assert_eq!(s.active(), 1);
    }
    #[test]
    fn random_resolves_once_and_excludes_future_and_internal_characters() {
        for seed in 0..128 {
            let mut s = screen();
            s.seed = seed;
            s.cursors[0] = RANDOM_SLOT;
            s.command(
                0,
                SelectCommand {
                    confirm: true,
                    ..Default::default()
                },
            );
            let chosen = s.preview(0);
            s.update(40.0, SelectInput::default());
            assert_eq!(chosen, s.preview(0));
            assert!(PUBLIC_ROSTER.contains(&chosen));
        }
    }
    #[test]
    fn future_slots_cannot_confirm() {
        let mut s = screen();
        s.cursors[0] = 7;
        assert_eq!(
            s.command(
                0,
                SelectCommand {
                    confirm: true,
                    ..Default::default()
                }
            ),
            SelectAction::Reject
        );
        assert!(!s.ready(0));
    }

    #[test]
    fn two_devices_confirming_the_last_owner_cannot_also_launch_that_frame() {
        let mut s = screen();
        s.command(
            0,
            SelectCommand {
                confirm: true,
                ..Default::default()
            },
        );
        let confirm = SelectCommand {
            confirm: true,
            ..Default::default()
        };
        let action = s.update(
            0.0,
            SelectInput {
                shared: confirm,
                players: [SelectCommand::default(), confirm],
                ..Default::default()
            },
        );
        assert_eq!(action, SelectAction::Confirm);
        assert!(s.both_ready());
        assert_eq!(
            s.update(
                0.0,
                SelectInput {
                    shared: confirm,
                    ..Default::default()
                }
            ),
            SelectAction::Launch
        );
    }

    #[test]
    fn local_back_does_not_unlock_the_other_players_pick() {
        let mut s = screen();
        s.command(
            0,
            SelectCommand {
                confirm: true,
                ..Default::default()
            },
        );
        s.update(
            0.0,
            SelectInput {
                players: [
                    SelectCommand::default(),
                    SelectCommand {
                        back: true,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        );
        assert!(s.ready(0));
        assert!(!s.ready(1));
    }
    #[test]
    fn independent_owners_can_confirm_in_one_frame_but_launch_needs_new_edge() {
        let mut s = screen();
        let confirm = SelectCommand {
            confirm: true,
            ..Default::default()
        };
        assert_eq!(
            s.update(
                0.0,
                SelectInput {
                    players: [confirm; 2],
                    ..Default::default()
                }
            ),
            SelectAction::Confirm
        );
        assert!(s.both_ready());
        assert_eq!(
            s.update(
                0.0,
                SelectInput {
                    launch: true,
                    ..Default::default()
                }
            ),
            SelectAction::Launch
        );
    }
    #[test]
    fn back_unlocks_before_leaving_and_mode_changes_clear_old_confirmations() {
        let mut s = screen();
        s.ready = [true; 2];
        assert_eq!(
            s.command(
                0,
                SelectCommand {
                    back: true,
                    ..Default::default()
                }
            ),
            SelectAction::Move
        );
        assert!(!s.ready(0));
        assert!(s.ready(1));
        s.update(
            0.0,
            SelectInput {
                cycle_mode: true,
                ..Default::default()
            },
        );
        assert_eq!(s.ready, [false; 2]);
        assert_eq!(s.mode, PlayMode::LocalDuel);
    }
    #[test]
    fn scene_entry_discards_the_previous_screens_confirm() {
        let mut s = CharacterSelect::new(
            [CharacterId::Rust; 2],
            ArenaId::Sirius,
            PlayMode::AgainstCpu,
            0,
        );
        s.update(
            0.0,
            SelectInput {
                shared: SelectCommand {
                    confirm: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        assert_eq!(s.ready, [false; 2]);
    }
}
