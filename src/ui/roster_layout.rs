//! Shares the character selection geometry between rendering and pointer input.
//!
//! Bounds use the game's fixed 1280 by 720 presentation canvas.

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Bounds {
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x as f32
            && x < (self.x + self.w) as f32
            && y >= self.y as f32
            && y < (self.y + self.h) as f32
    }
}
pub const MODE: Bounds = Bounds {
    x: 350,
    y: 533,
    w: 580,
    h: 40,
};
pub const ARENA: Bounds = Bounds {
    x: 350,
    y: 582,
    w: 580,
    h: 48,
};
pub const LAUNCH: Bounds = Bounds {
    x: 440,
    y: 646,
    w: 400,
    h: 42,
};
pub const BACK: Bounds = Bounds {
    x: 30,
    y: 646,
    w: 200,
    h: 42,
};
pub fn preview(owner: usize) -> Bounds {
    Bounds {
        x: if owner == 0 { 30 } else { 952 },
        y: 140,
        w: 298,
        h: 432,
    }
}
pub fn cell(index: usize) -> Bounds {
    Bounds {
        x: 350 + (index % 4) as i32 * 147,
        y: 220 + (index / 4) as i32 * 151,
        w: 139,
        h: 140,
    }
}
pub fn hovered_cell(x: f32, y: f32) -> Option<usize> {
    (0..8).find(|&i| cell(i).contains(x, y))
}
