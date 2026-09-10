//! Keeps prepared menu graphics alive while an outer host runs another application.
//!
//! System: Fighting application boundary. Requests are opaque to this domain;
//! only the host knows how to launch the story, preserving isolated builds.

use super::App;
use crate::engine::assets::GameAssets;
use raylib::prelude::*;

/// Why the prepared menu returned control to its window owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuExit {
    /// The user selected the optional story entry.
    Story,
    /// The user quit or closed the shared window.
    Closed,
}

/// Owns menu state and textures across visits to a hosted story.
pub struct PreparedMenu {
    app: App,
    assets: GameAssets,
}

impl PreparedMenu {
    /// Loads graphics without opening the fighting audio device.
    pub(super) fn new(window: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut app = App::at_main_menu();
        app.preferences_menu.set_story_available(true);
        Self {
            app,
            assets: GameAssets::load(window, thread),
        }
    }

    /// Runs the menu until a host request; graphics stay owned for the return trip.
    pub fn run_hosted(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) -> MenuExit {
        self.app.preferences_menu.ignore_next_input();
        self.app.accumulator = 0.0;
        self.app.run_with_assets(window, thread, &self.assets)
    }

    /// Runs once for older hosts that do not service optional continuation requests.
    pub fn run(mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        self.app.preferences_menu.set_story_available(false);
        self.run_hosted(window, thread);
    }
}
