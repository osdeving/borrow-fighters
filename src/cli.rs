//! Parses startup options for the prototype executable.
//!
//! System: Application bootstrap. The CLI selects startup mode before Raylib is
//! initialized; gameplay commands stay in engine input modules.
//!
//! The CLI stays deliberately small until the prototype needs a real command
//! framework.

use std::fmt::{Display, Formatter};

use crate::characters::CharacterId;
use crate::scenes::combat_lab::{CombatLabMove, CombatLabOptions, CombatLabPose};

/// Startup mode selected from command-line arguments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LaunchMode {
    #[default]
    Game,
    CombatLab(CombatLabOptions),
}

/// Parsed startup options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--fight" | "--skip-menu" => {
                    start_fight = true;
                }
                "--lab" => {
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
                "--help" | "-h" => {
                    return Err(CliError::new(usage()));
                }
                _ => return Err(CliError::new(format!("unknown argument '{arg}'"))),
            }
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
    "Usage:\n  cargo run\n  cargo run -- --fight --p1 go --p2 duke\n  cargo run -- --lab combat --character rust --move light_punch\n  cargo run -- --lab combat --character duke --pose block\n  cargo run -- --lab combat --character go --move kick"
}
