//! Starts the external actor laboratory with the same production packs as gameplay.
//!
//! The library boundary owns content validation, the window and review capture.

fn main() {
    if let Err(error) = borrow_fighters::adventure::lab_app::run(std::env::args()) {
        eprintln!("Actor lab: {error}");
        std::process::exit(1);
    }
}
