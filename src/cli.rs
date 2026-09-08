//! Parses startup options for the prototype executable.
//!
//! System: Application bootstrap. The CLI selects startup mode before Raylib is
//! initialized; gameplay commands stay in engine input modules.
//!
//! The CLI stays deliberately small until the prototype needs a real command
//! framework.

use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use crate::characters::CharacterId;
use crate::scenes::{
    combat_lab::{CombatLabMove, CombatLabOptions, CombatLabPose},
    sprite_viewer::SpriteViewerOptions,
};

/// Startup mode selected from command-line arguments.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum LaunchMode {
    #[default]
    Game,
    CombatLab(CombatLabOptions),
    MoveShowcase(ShowcaseLaunchOptions),
    SpriteViewer(SpriteViewerOptions),
}

/// Selects a contextual demonstration and optional fixed replay at startup.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShowcaseLaunchOptions {
    pub character: CharacterId,
    pub selected_move: CombatLabMove,
    pub repeat_current: bool,
    pub sides_reversed: bool,
}

/// Parsed startup options.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LaunchOptions {
    pub mode: LaunchMode,
    pub match_options: MatchOptions,
    pub start_fight: bool,
}

/// Match setup selected before the app creates the first world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MatchOptions {
    pub player_one: CharacterId,
    pub player_two: CharacterId,
}

impl Default for MatchOptions {
    fn default() -> Self {
        Self {
            player_one: CharacterId::Rust,
            player_two: CharacterId::Duke,
        }
    }
}

/// Error returned for unsupported command-line arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliError {
    message: String,
}

impl LaunchOptions {
    /// Parses process arguments, including the executable name.
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, CliError> {
        let mut args = args.into_iter();
        let _program = args.next();
        let mut mode = LaunchMode::Game;
        let mut lab = CombatLabOptions::default();
        let mut match_options = MatchOptions::default();
        let mut start_fight = false;
        let mut sprite_viewer_requested = false;
        let mut sprite_viewer_manifest = None;
        let mut sprite_viewer_clip = None;
        let mut sprite_viewer_character_requested = false;
        let mut showcase_requested = false;
        let mut repeat_current = false;
        let mut sides_reversed = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--showcase" => showcase_requested = true,
                "--repeat" => repeat_current = true,
                "--reverse" => sides_reversed = true,
                "--fight" | "--skip-menu" => {
                    start_fight = true;
                }
                "--lab" => {
                    if sprite_viewer_requested {
                        return Err(CliError::new("cannot combine --lab with sprite viewer"));
                    }
                    let Some(kind) = args.next() else {
                        return Err(CliError::new("--lab requires a value"));
                    };
                    if kind != "combat" {
                        return Err(CliError::new(format!("unsupported lab '{kind}'")));
                    }
                    mode = LaunchMode::CombatLab(lab);
                }
                "--character" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--character requires a value"));
                    };
                    lab.character = CharacterId::from_cli(&value)
                        .ok_or_else(|| CliError::new(format!("unknown character '{value}'")))?;
                    sprite_viewer_character_requested = true;
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        mode = LaunchMode::CombatLab(lab);
                    }
                }
                "--p1" | "--player-one" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--p1 requires a value"));
                    };
                    match_options.player_one = CharacterId::from_cli(&value).ok_or_else(|| {
                        CliError::new(format!("unknown player one character '{value}'"))
                    })?;
                }
                "--p2" | "--player-two" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--p2 requires a value"));
                    };
                    match_options.player_two = CharacterId::from_cli(&value).ok_or_else(|| {
                        CliError::new(format!("unknown player two character '{value}'"))
                    })?;
                }
                "--move" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--move requires a value"));
                    };
                    lab.selected_move = CombatLabMove::from_cli(&value)
                        .ok_or_else(|| CliError::new(format!("unknown move '{value}'")))?;
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        mode = LaunchMode::CombatLab(lab);
                    }
                }
                "--pose" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--pose requires a value"));
                    };
                    lab.pose = CombatLabPose::from_cli(&value)
                        .ok_or_else(|| CliError::new(format!("unknown pose '{value}'")))?;
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        mode = LaunchMode::CombatLab(lab);
                    }
                }
                "--tool" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--tool requires a value"));
                    };
                    if value != "sprite-viewer" {
                        return Err(CliError::new(format!("unsupported tool '{value}'")));
                    }
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        return Err(CliError::new(
                            "cannot combine --tool sprite-viewer with --lab",
                        ));
                    }
                    sprite_viewer_requested = true;
                }
                "--manifest" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--manifest requires a value"));
                    };
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        return Err(CliError::new("cannot combine --manifest with --lab"));
                    }
                    sprite_viewer_requested = true;
                    sprite_viewer_manifest = Some(PathBuf::from(value));
                }
                "--clip" => {
                    let Some(value) = args.next() else {
                        return Err(CliError::new("--clip requires a value"));
                    };
                    if matches!(mode, LaunchMode::CombatLab(_)) {
                        return Err(CliError::new("cannot combine --clip with --lab"));
                    }
                    sprite_viewer_requested = true;
                    sprite_viewer_clip = Some(value);
                }
                "--help" | "-h" => {
                    return Err(CliError::new(usage()));
                }
                _ => return Err(CliError::new(format!("unknown argument '{arg}'"))),
            }
        }

        if showcase_requested {
            if sprite_viewer_requested || matches!(mode, LaunchMode::CombatLab(_)) || start_fight {
                return Err(CliError::new(
                    "cannot combine --showcase with --fight, --lab or sprite viewer",
                ));
            }
            if lab.pose != CombatLabPose::Move {
                return Err(CliError::new(
                    "--pose belongs to Combat Lab; showcase demonstrates attacks and defense automatically",
                ));
            }
            mode = LaunchMode::MoveShowcase(ShowcaseLaunchOptions {
                character: lab.character,
                selected_move: lab.selected_move,
                repeat_current,
                sides_reversed,
            });
        } else if repeat_current || sides_reversed {
            return Err(CliError::new("--repeat and --reverse require --showcase"));
        }

        if matches!(mode, LaunchMode::CombatLab(_) | LaunchMode::MoveShowcase(_))
            && lab.character == CharacterId::Go
            && lab.selected_move == CombatLabMove::SignatureSpecial
        {
            return Err(CliError::new(
                "Go has no signature special in this roster update",
            ));
        }

        if sprite_viewer_requested {
            let Some(manifest_path) = sprite_viewer_manifest else {
                return Err(CliError::new(
                    "--tool sprite-viewer requires --manifest <path>",
                ));
            };
            let character = if sprite_viewer_character_requested {
                Some(lab.character)
            } else {
                infer_sprite_viewer_character(&manifest_path)
            };
            mode = LaunchMode::SpriteViewer(SpriteViewerOptions {
                manifest_path,
                initial_clip: sprite_viewer_clip,
                character,
                selected_move: lab.selected_move,
            });
        }

        Ok(Self {
            mode,
            match_options,
            start_fight,
        })
    }
}

impl CliError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)?;
        if self.message != usage() {
            write!(formatter, "\n\n{}", usage())?;
        }
        Ok(())
    }
}

impl std::error::Error for CliError {}

fn usage() -> &'static str {
    "Usage:\n  cargo run\n  cargo run -- --showcase --character rust --move anti_air --repeat --reverse\n  cargo run -- --fight --p1 cpp --p2 python\n  cargo run -- --lab combat --character rust --move light_punch\n  cargo run -- --lab combat --character duke --pose block\n  cargo run -- --lab combat --character cpp --move kick\n  cargo run -- --tool sprite-viewer --manifest assets/placeholder/python-fighter.sprite.json --clip idle --character python --move projectile"
}

fn infer_sprite_viewer_character(path: &std::path::Path) -> Option<CharacterId> {
    let raw = path.to_string_lossy().to_ascii_lowercase();
    if raw.contains("rust") {
        Some(CharacterId::Rust)
    } else if raw.contains("duke") || raw.contains("java") {
        Some(CharacterId::Duke)
    } else if raw.contains("go") || raw.contains("gopher") || raw.contains("golang") {
        Some(CharacterId::Go)
    } else if raw.contains("langc") || raw.contains("c-fighter") || raw.contains("/c-") {
        Some(CharacterId::C)
    } else if raw.contains("python") || raw.contains("/py-") {
        Some(CharacterId::Python)
    } else if raw.contains("cpp") || raw.contains("c++") || raw.contains("cplusplus") {
        Some(CharacterId::Cpp)
    } else {
        None
    }
}
