//! Owns C++'s independent Rua Augusta chapter and its narrative checkpoints.
//!
//! System: Adventure chapter. Content and fixed-step direction depend on the
//! production actor contract; resource lifetimes and files belong to the app.

pub mod ambient;
pub mod chapter;
pub mod checkpoint;
pub mod cinema;
pub mod store;
pub mod texts;
pub mod world;

pub use chapter::{ArrivalPose, Chapter, ChapterInput, NpcPose, Phase};
pub use checkpoint::{Checkpoint, CheckpointStage};
pub use texts::Texts;
pub use world::{ChapterSpec, World};

/// The entry file for this playable chapter; other protagonists keep their own content.
pub const CONTENT_PATH: &str = "assets/adventure/chapters/cpp-augusta/chapter.json";
