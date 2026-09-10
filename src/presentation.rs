//! Connects the story presentation to the fighting menu in one host-owned window.
//!
//! System: Optional application composition. Only application APIs cross this
//! boundary; neither game imports the host or shares specific gameplay state.

use crate::{
    adventure::app::{self as adventure, Options, SessionExit},
    app::App,
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
};
use std::error::Error;

enum Entry {
    Menu,
    Story(Options),
    Help,
}

impl Entry {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        let args: Vec<String> = args.into_iter().collect();
        if args.iter().skip(1).any(|s| s == "--help" || s == "-h") {
            return Ok(Self::Help);
        }
        if args.iter().skip(1).any(|s| s == "--menu") {
            if args.len() != 2 {
                return Err("--menu is used alone; scene/capture flags belong to the story".into());
            }
            return Ok(Self::Menu);
        }
        Ok(match Options::parse(args)? {
            Some(options) => Self::Story(options),
            None => Self::Help,
        })
    }
}

/// Runs Ada through the presentation and then the main menu, or opens that menu.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let entry = Entry::parse(args)?;
    if matches!(entry, Entry::Help) {
        println!(
            "Borrow Fighters — história e menu\n\n\
             cargo run --features adventure --bin borrow-story\n\n\
             --menu                    Open the main menu directly\n\
             --start ada|morning|encounter|opening\n\
                                       Select story entry, then continue to menu\n\
             --texts PATH              Editable adventure text (F5 reload)\n\
             --capture DIR / --review DIR\n\
                                       Capture adventure before the menu\n\
             --frames N                Exit if the adventure reaches this frame limit\n\
             --mute / --hidden         Adventure audio / hidden review window\n\n\
             Modo História is reserved for the continuation and has no action yet."
        );
        return Ok(());
    }
    let mut builder = raylib::init();
    builder
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters")
        .msaa_4x();
    if matches!(&entry, Entry::Story(options) if options.hidden_window()) {
        builder.hidden();
    }
    let (mut window, thread) = builder.build();
    window.set_exit_key(None);
    window.show_cursor();
    // Prepare fighting textures before Ada starts, so the final fade never
    // waits for the roster's large atlases. The opaque menu owns its resources;
    // adventure cannot see them, and its audio device still runs independently.
    let menu = App::prepare_main_menu(&mut window, &thread);
    if let Entry::Story(options) = entry
        && !opens_menu(adventure::run_in_window(&mut window, &thread, options)?)
    {
        return Ok(());
    }
    // Capture flags hide only the adventure; a completed run must never leave
    // the player in an invisible, live menu with no way to interact with it.
    window.clear_window_state(raylib::prelude::WindowState::default().set_window_hidden(true));
    window.set_window_title(&thread, "Borrow Fighters");
    menu.run(&mut window, &thread);
    Ok(())
}

fn opens_menu(exit: SessionExit) -> bool {
    exit == SessionExit::Completed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_and_frame_limits_never_launch_another_mode() {
        assert!(!opens_menu(SessionExit::Closed));
        assert!(!opens_menu(SessionExit::FrameLimit));
        assert!(opens_menu(SessionExit::Completed));
    }

    #[test]
    fn explicit_menu_entry_cannot_silently_discard_story_options() {
        for args in [
            vec!["story", "--menu", "--start", "opening"],
            vec!["story", "--frames", "0"],
            vec!["story", "--start", "invalid"],
        ] {
            assert!(Entry::parse(args.into_iter().map(str::to_owned)).is_err());
        }
        assert!(matches!(
            Entry::parse(["story", "--menu"].map(str::to_owned)).unwrap(),
            Entry::Menu
        ));
        assert!(matches!(
            Entry::parse(["story"].map(str::to_owned)).unwrap(),
            Entry::Story(_)
        ));
    }
}
