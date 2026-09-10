//! Starts the optional Rust story adventure without the fighting application.
//!
//! System: Adventure application boundary. Window ownership and gameplay live
//! in the adventure application, keeping this entrypoint small.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = borrow_fighters::adventure::app::run(std::env::args()) {
        eprintln!("Adventure: {error}");
        std::process::exit(1);
    }
}
