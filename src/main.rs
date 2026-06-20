//! Starts the Borrow Fighters greybox prototype.
//!
//! This binary stays thin: it creates the Raylib window and hands control to
//! the application loop.

use borrow_fighters::app::App;
use borrow_fighters::config::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

fn main() {
    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .build();

    App::default().run(&mut raylib, &thread);
}
