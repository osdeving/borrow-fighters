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

        while let Some(arg) = args.next() {
            match arg.as_str() {
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

        Ok(Self { mode })
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
    "Usage:\n  cargo run\n  cargo run -- --lab combat --character rust --move light_punch\n  cargo run -- --lab combat --character duke --pose block"
}
