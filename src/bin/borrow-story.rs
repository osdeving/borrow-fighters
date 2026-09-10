//! Starts the optional story-to-menu presentation in one window.
//!
//! System: Composition entrypoint. Each game keeps its independent application;
//! the presentation host only sequences their public session boundaries.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = borrow_fighters::presentation::run(std::env::args()) {
        eprintln!("Borrow Fighters: {error}");
        std::process::exit(1);
    }
}
