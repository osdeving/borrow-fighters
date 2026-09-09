//! Owns the pause and result menus without advancing the fight.
//!
//! Shared row geometry keeps pointer input aligned with the overlay renderer.

use crate::config::screen_px;
use crate::math::rect::Rect;
use crate::math::vec2::Vec2;

/// The two overlays that can interrupt a running or completed match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchFlowKind {
    Pause,
    Result,
}

/// A choice applied by the app after closing the overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchFlowAction {
    Resume,
    Restart,
    CharacterSelect,
    Menu,
}

impl MatchFlowAction {
    /// Player-facing text, including the rematch wording after a result.
    pub const fn label(self, kind: MatchFlowKind) -> &'static str {
        match (self, kind) {
            (Self::Resume, _) => "Continuar",
            (Self::Restart, MatchFlowKind::Pause) => "Reiniciar",
            (Self::Restart, MatchFlowKind::Result) => "Revanche",
            (Self::CharacterSelect, _) => "Trocar personagens",
            (Self::Menu, _) => "Menu",
        }
    }
}

/// Commands for one UI frame; the app maps keyboard and controller edges here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MatchFlowInput {
    pub up: bool,
    pub down: bool,
    pub confirm: bool,
    pub back: bool,
    pub hovered_row: Option<usize>,
    pub pointer_moved: bool,
    pub click: bool,
}

impl MatchFlowInput {
    fn is_neutral(self) -> bool {
        !(self.up || self.down || self.confirm || self.back || self.click)
    }
}

const PAUSE_ACTIONS: [MatchFlowAction; 4] = [
    MatchFlowAction::Resume,
    MatchFlowAction::Restart,
    MatchFlowAction::CharacterSelect,
    MatchFlowAction::Menu,
];
const RESULT_ACTIONS: [MatchFlowAction; 3] = [
    MatchFlowAction::Restart,
    MatchFlowAction::CharacterSelect,
    MatchFlowAction::Menu,
];

/// Selection and an independent animation clock for a match overlay.
#[derive(Clone, Debug)]
pub struct MatchFlow {
    kind: MatchFlowKind,
    selected: usize,
    elapsed_seconds: f32,
    accepting_input: bool,
}

impl MatchFlow {
    /// Opens pause with Continue selected and the opening input still blocked.
    pub const fn pause() -> Self {
        Self::new(MatchFlowKind::Pause)
    }

    /// Opens the result with Rematch selected and the opening input blocked.
    pub const fn result() -> Self {
        Self::new(MatchFlowKind::Result)
    }

    const fn new(kind: MatchFlowKind) -> Self {
        Self {
            kind,
            selected: 0,
            elapsed_seconds: 0.0,
            accepting_input: false,
        }
    }

    pub const fn kind(&self) -> MatchFlowKind {
        self.kind
    }

    pub const fn selected(&self) -> usize {
        self.selected
    }

    pub const fn elapsed_seconds(&self) -> f32 {
        self.elapsed_seconds
    }

    /// Advances presentation only, including while the world is paused.
    pub fn tick(&mut self, dt: f32) {
        if dt.is_finite() && dt > 0.0 {
            self.elapsed_seconds += dt.min(0.25);
        }
    }

    pub const fn actions(&self) -> &'static [MatchFlowAction] {
        match self.kind {
            MatchFlowKind::Pause => &PAUSE_ACTIONS,
            MatchFlowKind::Result => &RESULT_ACTIONS,
        }
    }

    /// Consumes the opening edge until a neutral frame prevents Start/Esc reuse.
    ///
    /// Pointer movement changes selection; an idle pointer does not override
    /// keyboard navigation. Clicking outside a row never confirms a selection.
    pub fn handle_input(&mut self, input: MatchFlowInput) -> Option<MatchFlowAction> {
        if !self.accepting_input {
            self.accepting_input = input.is_neutral();
            return None;
        }

        let hovered = input.hovered_row.filter(|row| *row < self.actions().len());
        if (input.pointer_moved || input.click)
            && let Some(row) = hovered
        {
            self.selected = row;
        }
        if input.up != input.down {
            let count = self.actions().len();
            self.selected = if input.up {
                (self.selected + count - 1) % count
            } else {
                (self.selected + 1) % count
            };
        }

        let action = if input.back {
            Some(match self.kind {
                MatchFlowKind::Pause => MatchFlowAction::Resume,
                MatchFlowKind::Result => MatchFlowAction::Menu,
            })
        } else if input.click {
            hovered.map(|row| self.actions()[row])
        } else if input.confirm {
            Some(self.actions()[self.selected])
        } else {
            None
        };
        if action.is_some() {
            self.accepting_input = false;
        }
        action
    }

    /// Stable screen-space bounds, shared by drawing and hit-testing.
    pub fn row_bounds(&self, index: usize) -> Option<Rect> {
        if index >= self.actions().len() {
            return None;
        }
        let (x, y, width, height) = match self.kind {
            MatchFlowKind::Pause => (268, 220 + index as i32 * 44, 424, 36),
            MatchFlowKind::Result => (90 + index as i32 * 268, 477, 248, 31),
        };
        Some(Rect::new(
            screen_px(x) as f32,
            screen_px(y) as f32,
            screen_px(width) as f32,
            screen_px(height) as f32,
        ))
    }

    pub fn row_at(&self, point: Vec2) -> Option<usize> {
        (0..self.actions().len()).find(|index| {
            let bounds = self.row_bounds(*index).expect("known menu row");
            point.x >= bounds.x
                && point.x < bounds.right()
                && point.y >= bounds.y
                && point.y < bounds.bottom()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FLOOR_Y, WINDOW_HEIGHT, WINDOW_WIDTH};

    fn armed(mut flow: MatchFlow) -> MatchFlow {
        assert_eq!(flow.handle_input(MatchFlowInput::default()), None);
        flow
    }

    #[test]
    fn opening_edge_is_blocked_until_a_neutral_frame() {
        for mut flow in [MatchFlow::pause(), MatchFlow::result()] {
            let opening = MatchFlowInput {
                confirm: true,
                back: true,
                ..Default::default()
            };
            assert_eq!(flow.handle_input(opening), None);
            assert_eq!(flow.handle_input(opening), None);
            assert_eq!(flow.handle_input(MatchFlowInput::default()), None);
            assert_eq!(
                flow.handle_input(MatchFlowInput {
                    confirm: true,
                    ..Default::default()
                }),
                Some(flow.actions()[0])
            );
        }
    }

    #[test]
    fn back_resumes_pause_and_leaves_result_for_the_menu() {
        let back = MatchFlowInput {
            back: true,
            confirm: true,
            ..Default::default()
        };
        assert_eq!(
            armed(MatchFlow::pause()).handle_input(back),
            Some(MatchFlowAction::Resume)
        );
        assert_eq!(
            armed(MatchFlow::result()).handle_input(back),
            Some(MatchFlowAction::Menu)
        );
    }

    #[test]
    fn keyboard_navigation_wraps_each_menu_without_idle_pointer_interference() {
        for mut flow in [armed(MatchFlow::pause()), armed(MatchFlow::result())] {
            flow.handle_input(MatchFlowInput {
                up: true,
                hovered_row: Some(0),
                ..Default::default()
            });
            assert_eq!(flow.selected(), flow.actions().len() - 1);
            flow.handle_input(MatchFlowInput {
                hovered_row: Some(0),
                ..Default::default()
            });
            assert_eq!(flow.selected(), flow.actions().len() - 1);
            flow.handle_input(MatchFlowInput {
                down: true,
                ..Default::default()
            });
            assert_eq!(flow.selected(), 0);
        }
    }

    #[test]
    fn click_requires_a_valid_row_and_selects_the_clicked_action() {
        let mut flow = armed(MatchFlow::pause());
        assert_eq!(
            flow.handle_input(MatchFlowInput {
                click: true,
                ..Default::default()
            }),
            None
        );
        assert_eq!(
            flow.handle_input(MatchFlowInput {
                click: true,
                hovered_row: Some(99),
                ..Default::default()
            }),
            None
        );
        assert_eq!(
            flow.handle_input(MatchFlowInput {
                click: true,
                hovered_row: Some(2),
                ..Default::default()
            }),
            Some(MatchFlowAction::CharacterSelect)
        );
        assert_eq!(flow.selected(), 2);
    }

    #[test]
    fn result_choices_stay_below_fighters_and_share_their_hit_regions() {
        let flow = MatchFlow::result();
        for index in 0..flow.actions().len() {
            let row = flow.row_bounds(index).unwrap();
            assert!(row.y >= FLOOR_Y);
            assert!(row.x >= 0.0 && row.right() <= WINDOW_WIDTH as f32);
            assert!(row.bottom() <= WINDOW_HEIGHT as f32);
            assert_eq!(flow.row_at(row.center()), Some(index));
            if let Some(next) = flow.row_bounds(index + 1) {
                assert!(!row.intersects(next));
            }
        }
        assert_eq!(flow.row_at(Vec2::ZERO), None);
        assert_eq!(flow.row_bounds(3), None);
    }

    #[test]
    fn animation_clock_ignores_invalid_time_and_bounds_a_stalled_frame() {
        let mut flow = MatchFlow::pause();
        for dt in [-1.0, f32::NAN, f32::INFINITY] {
            flow.tick(dt);
        }
        assert_eq!(flow.elapsed_seconds(), 0.0);
        flow.tick(10.0);
        assert_eq!(flow.elapsed_seconds(), 0.25);
    }
}
