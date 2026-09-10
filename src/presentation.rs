//! Connects the story presentation to the fighting menu in one host-owned window.
//!
//! System: Optional application composition. Only application APIs cross this
//! boundary; neither game imports the host or shares specific gameplay state.

use crate::{
    adventure::app::{self as adventure, CampaignExit, Options, SessionExit},
    app::{App, MenuExit},
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
};
use std::error::Error;

enum Entry {
    Menu,
    Automatic(Options),
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
        let automatic = args.len() == 1;
        Ok(match Options::parse(args)? {
            Some(options) if automatic => Self::Automatic(options),
            Some(options) => Self::Story(options),
            None => Self::Help,
        })
    }
}

/// Runs first-time presentation and routes repeat visits through the retained menu.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let entry = Entry::parse(args)?;
    if matches!(entry, Entry::Help) {
        println!(
            "Borrow Fighters — história e menu\n\n\
             cargo run\n\n\
             --menu                    Open the main menu directly\n\
             --start ada|morning|encounter|opening|chapter\n\
                                       Explicit scene or chapter checkpoint entry\n\
             --texts PATH              Editable adventure text (F5 reload)\n\
             --capture DIR / --review DIR\n\
                                       Capture adventure before the menu\n\
             --frames N                Exit if the adventure reaches this frame limit\n\
             --mute / --hidden         Adventure audio / hidden review window\n\n\
             First launch shows Ada/Rust; later launches open the menu.\n\
             Modo História offers Continue, New Chapter and Replay Prologue."
        );
        return Ok(());
    }
    let mut builder = raylib::init();
    builder
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters")
        .msaa_4x();
    if matches!(&entry, Entry::Story(options)|Entry::Automatic(options) if options.hidden_window())
    {
        builder.hidden();
    }
    let (mut window, thread) = builder.build();
    window.set_exit_key(None);
    window.show_cursor();
    // Prepare fighting textures before Ada starts, so the final fade never
    // waits for the roster's large atlases. The opaque menu owns its resources;
    // adventure cannot see them, and its audio device still runs independently.
    let mut menu = App::prepare_main_menu(&mut window, &thread);
    let seen = if matches!(entry, Entry::Automatic(_)) {
        match adventure::prologue_seen() {
            Ok(seen) => seen,
            Err(error) => {
                show_error(
                    &mut window,
                    &thread,
                    "Não foi possível ler o progresso.",
                    &error.to_string(),
                );
                // An invalid profile is kept intact. The campaign screen can
                // explain recovery; no automatic replay writes over this file.
                true
            }
        }
    } else {
        false
    };
    let mut flow = initial_flow(entry, seen);
    while !window.window_should_close() {
        flow = match flow {
            Flow::Menu => {
                show_window(&mut window, &thread);
                match menu.run_hosted(&mut window, &thread) {
                    MenuExit::Story => Flow::Campaign(Options::default()),
                    MenuExit::Closed => break,
                }
            }
            Flow::Prologue(options) => {
                let developer_run = options.hidden_window();
                match adventure::run_in_window(&mut window, &thread, options) {
                    Ok(exit) if opens_menu(exit) => Flow::Menu,
                    Ok(_) => break,
                    Err(error) => {
                        if developer_run {
                            return Err(error);
                        }
                        show_error(
                            &mut window,
                            &thread,
                            "A história não pôde ser concluída.",
                            &error.to_string(),
                        );
                        Flow::Menu
                    }
                }
            }
            Flow::Campaign(options) => {
                let developer_run = options.hidden_window();
                match adventure::run_campaign_in_window(&mut window, &thread, options) {
                    Ok(CampaignExit::Menu) => Flow::Menu,
                    Ok(CampaignExit::ReplayPrologue) => Flow::Prologue(Options::prologue()),
                    Ok(CampaignExit::Closed | CampaignExit::FrameLimit) => break,
                    Err(error) => {
                        if developer_run {
                            return Err(error);
                        }
                        show_error(
                            &mut window,
                            &thread,
                            "Não foi possível abrir o capítulo.",
                            &error.to_string(),
                        );
                        Flow::Menu
                    }
                }
            }
        };
    }
    Ok(())
}

enum Flow {
    Menu,
    Prologue(Options),
    Campaign(Options),
}

fn initial_flow(entry: Entry, seen: bool) -> Flow {
    match entry {
        Entry::Automatic(_) if seen => Flow::Menu,
        Entry::Story(options) if options.is_chapter() => Flow::Campaign(options),
        Entry::Story(options) | Entry::Automatic(options) => Flow::Prologue(options),
        Entry::Menu | Entry::Help => Flow::Menu,
    }
}

fn show_window(window: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) {
    window.clear_window_state(raylib::prelude::WindowState::default().set_window_hidden(true));
    window.set_window_title(thread, "Borrow Fighters");
}

fn show_error(
    window: &mut raylib::RaylibHandle,
    thread: &raylib::RaylibThread,
    title: &str,
    detail: &str,
) {
    use raylib::prelude::*;
    eprintln!("{title} {detail}");
    show_window(window, thread);
    window.set_target_fps(60);
    let detail: String = detail.chars().take(160).collect();
    let mut frames = 0;
    while !window.window_should_close() {
        if frames > 2
            && (window.is_key_pressed(KeyboardKey::KEY_ENTER)
                || window.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                || (window.is_gamepad_available(0)
                    && window.is_gamepad_button_pressed(
                        0,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
                    )))
        {
            break;
        }
        let mut draw = window.begin_drawing(thread);
        draw.clear_background(Color::new(13, 23, 28, 255));
        draw.draw_text(title, 70, 170, 28, Color::new(241, 184, 91, 255));
        for (line, chunk) in detail.chars().collect::<Vec<_>>().chunks(80).enumerate() {
            let text: String = chunk.iter().collect();
            draw.draw_text(&text, 70, 230 + line as i32 * 28, 18, Color::RAYWHITE);
        }
        draw.draw_text(
            "O arquivo de progresso foi preservado.",
            70,
            350,
            22,
            Color::RAYWHITE,
        );
        draw.draw_text(
            "Enter / A / Esc: voltar ao menu",
            70,
            405,
            20,
            Color::LIGHTGRAY,
        );
        frames += 1;
    }
}

fn opens_menu(exit: SessionExit) -> bool {
    matches!(exit, SessionExit::Completed | SessionExit::Skipped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_and_frame_limits_never_launch_another_mode() {
        assert!(!opens_menu(SessionExit::Closed));
        assert!(!opens_menu(SessionExit::FrameLimit));
        assert!(opens_menu(SessionExit::Completed));
        assert!(opens_menu(SessionExit::Skipped));
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
            Entry::Automatic(_)
        ));
    }

    #[test]
    fn returning_profiles_skip_only_the_implicit_startup() {
        for seen in [false, true] {
            let implicit = Entry::parse(["story"].map(str::to_owned)).unwrap();
            assert_eq!(matches!(initial_flow(implicit, seen), Flow::Menu), seen);
            for arguments in [
                vec!["story", "--start", "ada"],
                vec!["story", "--review", "capture"],
                vec!["story", "--capture", "capture"],
            ] {
                let explicit = Entry::parse(arguments.into_iter().map(str::to_owned)).unwrap();
                assert!(matches!(initial_flow(explicit, seen), Flow::Prologue(_)));
            }
            let chapter = Entry::parse(["story", "--start", "chapter"].map(str::to_owned)).unwrap();
            assert!(matches!(initial_flow(chapter, seen), Flow::Campaign(_)));
        }
    }
}
