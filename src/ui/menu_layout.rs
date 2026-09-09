//! Shares menu row coordinates between rendering and pointer hit testing.
//!
//! System: UI layout. These fixed prototype rectangles contain no Raylib state
//! or navigation behavior.

use crate::config::screen_px;
use crate::scenes::preferences::MenuPage;

/// Rectangle in rendered screen pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl MenuBounds {
    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x as f32
            && x < (self.x + self.width) as f32
            && y >= self.y as f32
            && y < (self.y + self.height) as f32
    }
}

/// Existing menu panel and its evenly spaced selectable rows.
#[derive(Clone, Copy, Debug)]
pub struct MenuLayout {
    pub panel: MenuBounds,
    pub row_step: i32,
    first_row: MenuBounds,
}

impl MenuLayout {
    /// Returns the layout used to both draw and click one menu page.
    pub fn for_page(page: MenuPage) -> Self {
        let (x, y, width, height, row_offset_x, row_offset_y, row_width, step, gap) = match page {
            MenuPage::Main => (302, 154, 356, 370, 42, 58, 272, 40, 4),
            MenuPage::Versus => (238, 96, 484, 432, 42, 96, 400, 56, 8),
            MenuPage::Training => (254, 126, 452, 390, 42, 106, 368, 56, 8),
            MenuPage::Lore => (54, 78, 852, 428, 46, 82, 294, 40, 8),
            MenuPage::Options => (144, 40, 672, 490, 48, 66, 576, 28, 2),
            MenuPage::HowToPlay => (34, 40, 892, 490, 244, 328, 404, 32, 4),
        };
        let panel = MenuBounds {
            x: screen_px(x),
            y: screen_px(y),
            width: screen_px(width),
            height: screen_px(height),
        };
        Self {
            panel,
            row_step: screen_px(step),
            first_row: MenuBounds {
                x: panel.x + screen_px(row_offset_x),
                y: panel.y + screen_px(row_offset_y),
                width: screen_px(row_width),
                height: screen_px(step) - screen_px(gap),
            },
        }
    }

    /// Visible bounds of a row, excluding the gap beneath it.
    pub fn row_bounds(self, index: usize) -> MenuBounds {
        MenuBounds {
            y: self.first_row.y + index as i32 * self.row_step,
            ..self.first_row
        }
    }

    /// Finds a visible row under the pointer; panel padding and gaps are inert.
    pub fn hovered_row(self, x: f32, y: f32, row_count: usize) -> Option<usize> {
        (0..row_count).find(|&index| self.row_bounds(index).contains(x, y))
    }
}
