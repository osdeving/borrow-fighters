//! Loads prototype assets at the Raylib boundary.
//!
//! Assets stay optional in the greybox phase so the game can still run with
//! procedural debug drawing if a local file is missing.

use raylib::core::{AsRawMut, text::RaylibFont};
use raylib::prelude::*;

use crate::characters::CharacterId;
use crate::engine::sprites::{
    C_BITSTREAM_PROJECTILE_PATH, C_FIGHTER_MANIFEST_PATH, C_START_MANIFEST_PATH,
    CPP_FIGHTER_MANIFEST_PATH, CPP_PLUSPLUS_PROJECTILE_PATH, DUKE_BEAN_PROJECTILE_PATH,
    DUKE_FIGHTER_MANIFEST_PATH, DUKE_START_MANIFEST_PATH, FIGHTER_SPRITESHEET_PATH,
    FighterSpriteClip, GO_CHANNEL_PROJECTILE_PATH, GO_FIGHTER_MANIFEST_PATH,
    GO_START_MANIFEST_PATH, PYTHON_DATA_PROJECTILE_PATH, PYTHON_FIGHTER_MANIFEST_PATH,
    PYTHON_START_MANIFEST_PATH, RUST_FIGHTER_MANIFEST_PATH, RUST_GEAR_PROJECTILE_PATH,
    RUST_START_MANIFEST_PATH, SpriteFrame, SpriteManifest,
};
use crate::game::arena::ArenaId;
use crate::lore::{LORE_BOOK_PATH, LoreBook};
use crate::runtime_paths::asset_path;

pub const ARENA_SIRIUS_PATH: &str = "assets/production/stage-life/arena-sirius-clean.png";
pub const ARENA_FORTALEZA_PATH: &str = "assets/placeholder/arena-fortaleza.png";
pub const ARENA_JAVA_STREET_PATH: &str = "assets/placeholder/arena-java-street.png";
pub const ARENA_BIOTIC_PATH: &str = "assets/placeholder/arena-biotic.png";
pub const ARENA_PORTO_DIGITAL_PATH: &str = "assets/placeholder/arena-porto-digital.png";
pub const ARENA_VALE_PINHAO_PATH: &str = "assets/placeholder/arena-vale-pinhao.png";
pub const CARAMELO_RUN_PATH: &str = "assets/production/stage-life/caramelo-run.png";
pub const JESSICA_GESTURE_PATH: &str = "assets/production/stage-life/jessica-gesture.png";

pub const COUNTDOWN_11_PATH: &str = "assets/placeholder/countdown-11.png";
pub const COUNTDOWN_10_PATH: &str = "assets/placeholder/countdown-10.png";
pub const COUNTDOWN_01_PATH: &str = "assets/placeholder/countdown-01.png";
pub const COUNTDOWN_FIGHT_PATH: &str = "assets/placeholder/countdown-fight.png";
pub const MENU_TITLE_PATH: &str = "assets/placeholder/menu-title-borrow-fighters.png";
pub const ROSTER_RUST_PATH: &str = "assets/placeholder/roster-rust.png";
pub const ROSTER_DUKE_PATH: &str = "assets/placeholder/roster-duke.png";
pub const ROSTER_C_PATH: &str = "assets/placeholder/roster-c.png";
pub const ROSTER_PYTHON_PATH: &str = "assets/placeholder/roster-python.png";
pub const ROSTER_CPP_PATH: &str = "assets/placeholder/roster-cpp.png";
const MENU_FONT: &[u8] = include_bytes!("../../assets/fonts/BarlowCondensed-SemiBold.ttf");
const LORE_FONT: &[u8] = include_bytes!("../../assets/fonts/Lora-Variable.ttf");
const LORE_BODY_FONT: &[u8] = include_bytes!("../../assets/fonts/Barlow-Regular.ttf");

/// Texture and metadata for one atlas-driven sprite set.
pub struct SpriteAtlasAsset {
    pub manifest: SpriteManifest,
    /// Reviewed baseline metadata; candidate artwork never changes combat boxes.
    pub combat_manifest: SpriteManifest,
    pub textures: Vec<SpriteAtlasTexture>,
}

/// One loaded texture referenced by a sprite manifest.
pub struct SpriteAtlasTexture {
    pub image: String,
    pub texture: Texture2D,
}

impl SpriteAtlasAsset {
    /// Returns the loaded texture used by a specific manifest frame.
    pub fn texture_for_frame(&self, frame: &SpriteFrame) -> Option<&Texture2D> {
        let image = self.manifest.frame_image(frame);
        self.textures
            .iter()
            .find(|texture| texture.image == image)
            .map(|texture| &texture.texture)
    }
}

/// Runtime textures used by the prototype renderer.
pub struct GameAssets {
    pub arenas: ArenaAssets,
    /// Separate authorial super props; they never supply combat geometry.
    pub duke_collector_poses: Option<Texture2D>,
    pub cpp_laptop: Option<Texture2D>,
    pub python_transform: Option<Texture2D>,
    pub python_serpent: Option<Texture2D>,
    pub python_revert: Option<Texture2D>,
    pub python_celebrate: Option<Texture2D>,
    pub cpp_footgun_comedy: Option<Texture2D>,
    pub cpp_footgun_barrage: Option<Texture2D>,
    pub garbage_items: Option<Texture2D>,
    /// Optional decorative actors, independent from fighter and collision data.
    pub caramelo_run: Option<Texture2D>,
    pub jessica_gesture: Option<Texture2D>,
    pub lore_book: LoreBook,
    pub menu_font: Option<Font>,
    pub lore_font: Option<Font>,
    pub lore_body_font: Option<Font>,
    pub menu_title: Option<Texture2D>,
    pub roster_portraits: RosterPortraitAssets,
    pub fighter_spritesheet: Option<Texture2D>,
    pub rust_fighter: Option<SpriteAtlasAsset>,
    pub rust_start: Option<SpriteAtlasAsset>,
    pub duke_fighter: Option<SpriteAtlasAsset>,
    pub duke_start: Option<SpriteAtlasAsset>,
    pub go_fighter: Option<SpriteAtlasAsset>,
    pub go_start: Option<SpriteAtlasAsset>,
    pub c_fighter: Option<SpriteAtlasAsset>,
    pub c_start: Option<SpriteAtlasAsset>,
    pub python_fighter: Option<SpriteAtlasAsset>,
    pub python_start: Option<SpriteAtlasAsset>,
    pub cpp_fighter: Option<SpriteAtlasAsset>,
    /// Separately authored signature VFX, keyed by the five playable fighters.
    pub signature_effects: Vec<(CharacterId, SpriteAtlasAsset)>,
    pub rust_projectile: Option<Texture2D>,
    pub duke_projectile: Option<Texture2D>,
    pub go_projectile: Option<Texture2D>,
    pub c_projectile: Option<Texture2D>,
    pub python_projectile: Option<Texture2D>,
    pub cpp_projectile: Option<Texture2D>,
    pub countdown_11: Option<Texture2D>,
    pub countdown_10: Option<Texture2D>,
    pub countdown_01: Option<Texture2D>,
    pub countdown_fight: Option<Texture2D>,
}

/// Arena background textures loaded at the Raylib boundary.
pub struct ArenaAssets {
    pub sirius: Option<Texture2D>,
    pub fortaleza: Option<Texture2D>,
    pub java_street: Option<Texture2D>,
    pub biotic: Option<Texture2D>,
    pub porto_digital: Option<Texture2D>,
    pub vale_pinhao: Option<Texture2D>,
}

/// Roster portrait textures loaded at the Raylib boundary.
pub struct RosterPortraitAssets {
    pub rust: Option<Texture2D>,
    pub duke: Option<Texture2D>,
    pub c: Option<Texture2D>,
    pub python: Option<Texture2D>,
    pub cpp: Option<Texture2D>,
}

impl RosterPortraitAssets {
    /// Returns the portrait for a selected character.
    pub fn get(&self, character: CharacterId) -> Option<&Texture2D> {
        match character {
            CharacterId::Rust => self.rust.as_ref(),
            CharacterId::Duke => self.duke.as_ref(),
            CharacterId::C => self.c.as_ref(),
            CharacterId::Python => self.python.as_ref(),
            CharacterId::Cpp => self.cpp.as_ref(),
            CharacterId::Go => None,
        }
    }
}

impl ArenaAssets {
    /// Returns the texture for a selected arena.
    pub fn get(&self, arena: ArenaId) -> Option<&Texture2D> {
        match arena {
            ArenaId::Sirius => self.sirius.as_ref(),
            ArenaId::Fortaleza => self.fortaleza.as_ref(),
            ArenaId::JavaStreet => self.java_street.as_ref(),
            ArenaId::BioTic => self.biotic.as_ref(),
            ArenaId::PortoDigital => self.porto_digital.as_ref(),
            ArenaId::ValeDoPinhao => self.vale_pinhao.as_ref(),
        }
    }
}

impl GameAssets {
    /// Finds the atlas used by a character's physical signature entities.
    pub fn signature_atlas(&self, character: CharacterId) -> Option<&SpriteAtlasAsset> {
        self.signature_effects
            .iter()
            .find(|(id, _)| *id == character)
            .map(|(_, atlas)| atlas)
    }

    /// Loads all optional prototype assets.
    pub fn load(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            cpp_laptop: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/cpp/laptop.png",
            ),
            python_transform: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/python/transform.png",
            ),
            python_serpent: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/python/serpent.png",
            ),
            python_revert: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/python/revert.png",
            ),
            python_celebrate: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/python/celebrate.png",
            ),
            duke_collector_poses: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/duke/collector-poses.png",
            ),
            cpp_footgun_comedy: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/cpp/footgun-comedy.png",
            ),
            cpp_footgun_barrage: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/cpp/footgun-barrage.png",
            ),
            garbage_items: load_smooth_texture_optional(
                raylib,
                thread,
                "assets/production/super-sequences/trash/garbage-items.png",
            ),
            arenas: ArenaAssets {
                sirius: load_smooth_texture_optional(raylib, thread, ARENA_SIRIUS_PATH),
                fortaleza: load_smooth_texture_optional(raylib, thread, ARENA_FORTALEZA_PATH),
                java_street: load_smooth_texture_optional(raylib, thread, ARENA_JAVA_STREET_PATH),
                biotic: load_smooth_texture_optional(raylib, thread, ARENA_BIOTIC_PATH),
                porto_digital: load_smooth_texture_optional(
                    raylib,
                    thread,
                    ARENA_PORTO_DIGITAL_PATH,
                ),
                vale_pinhao: load_smooth_texture_optional(raylib, thread, ARENA_VALE_PINHAO_PATH),
            },
            caramelo_run: load_smooth_texture_optional(raylib, thread, CARAMELO_RUN_PATH),
            jessica_gesture: load_smooth_texture_optional(raylib, thread, JESSICA_GESTURE_PATH),
            lore_book: LoreBook::load_or_default(asset_path(LORE_BOOK_PATH)),
            menu_font: load_ui_font(raylib, thread, MENU_FONT, "menu"),
            lore_font: load_ui_font(raylib, thread, LORE_FONT, "lore"),
            lore_body_font: load_ui_font(raylib, thread, LORE_BODY_FONT, "lore body"),
            menu_title: load_smooth_texture_optional(raylib, thread, MENU_TITLE_PATH),
            roster_portraits: RosterPortraitAssets {
                rust: load_smooth_texture_optional(raylib, thread, ROSTER_RUST_PATH),
                duke: load_smooth_texture_optional(raylib, thread, ROSTER_DUKE_PATH),
                c: load_smooth_texture_optional(raylib, thread, ROSTER_C_PATH),
                python: load_smooth_texture_optional(raylib, thread, ROSTER_PYTHON_PATH),
                cpp: load_smooth_texture_optional(raylib, thread, ROSTER_CPP_PATH),
            },
            fighter_spritesheet: load_texture_optional(raylib, thread, FIGHTER_SPRITESHEET_PATH),
            rust_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::Rust,
                RUST_FIGHTER_MANIFEST_PATH,
            ),
            rust_start: load_sprite_atlas_optional(raylib, thread, RUST_START_MANIFEST_PATH),
            duke_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::Duke,
                DUKE_FIGHTER_MANIFEST_PATH,
            ),
            duke_start: load_sprite_atlas_optional(raylib, thread, DUKE_START_MANIFEST_PATH),
            go_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::Go,
                GO_FIGHTER_MANIFEST_PATH,
            ),
            go_start: load_sprite_atlas_optional(raylib, thread, GO_START_MANIFEST_PATH),
            c_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::C,
                C_FIGHTER_MANIFEST_PATH,
            ),
            c_start: load_sprite_atlas_optional(raylib, thread, C_START_MANIFEST_PATH),
            python_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::Python,
                PYTHON_FIGHTER_MANIFEST_PATH,
            ),
            python_start: load_sprite_atlas_optional(raylib, thread, PYTHON_START_MANIFEST_PATH),
            cpp_fighter: load_fighter_atlas_optional(
                raylib,
                thread,
                CharacterId::Cpp,
                CPP_FIGHTER_MANIFEST_PATH,
            ),
            signature_effects: [
                CharacterId::Rust,
                CharacterId::Duke,
                CharacterId::C,
                CharacterId::Python,
                CharacterId::Cpp,
            ]
            .into_iter()
            .filter_map(|character| {
                let key = character.audio_key();
                let path = format!("assets/candidates/{key}/{key}-signature-fx.sprite.json");
                load_sprite_atlas_optional(raylib, thread, &path).map(|atlas| (character, atlas))
            })
            .collect(),
            rust_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::Rust,
                RUST_GEAR_PROJECTILE_PATH,
            ),
            duke_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::Duke,
                DUKE_BEAN_PROJECTILE_PATH,
            ),
            go_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::Go,
                GO_CHANNEL_PROJECTILE_PATH,
            ),
            c_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::C,
                C_BITSTREAM_PROJECTILE_PATH,
            ),
            python_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::Python,
                PYTHON_DATA_PROJECTILE_PATH,
            ),
            cpp_projectile: load_projectile_texture_optional(
                raylib,
                thread,
                CharacterId::Cpp,
                CPP_PLUSPLUS_PROJECTILE_PATH,
            ),
            countdown_11: load_smooth_texture_optional(raylib, thread, COUNTDOWN_11_PATH),
            countdown_10: load_smooth_texture_optional(raylib, thread, COUNTDOWN_10_PATH),
            countdown_01: load_smooth_texture_optional(raylib, thread, COUNTDOWN_01_PATH),
            countdown_fight: load_smooth_texture_optional(raylib, thread, COUNTDOWN_FIGHT_PATH),
        }
    }
}

fn load_ui_font(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    data: &[u8],
    label: &str,
) -> Option<Font> {
    // Raylib's default glyph subset excludes accented letters. Mipmaps average
    // the oversampled strokes before minification; bilinear alone still aliases
    // a 96 px atlas when a footer is displayed at 14 px.
    let glyphs: String = (32..=126)
        .chain(160..=255)
        .chain([
            0x2013, 0x2014, 0x2018, 0x2019, 0x201c, 0x201d, 0x2022, 0x2026,
        ])
        .filter(|&codepoint| codepoint != 0x00ad)
        .filter_map(char::from_u32)
        .collect();
    match raylib.load_font_from_memory(thread, ".ttf", data, 96, Some(&glyphs)) {
        Ok(mut font) => {
            // SAFETY: the live font owns this GPU texture. Raylib only updates
            // its mipmap count; no glyph pointers or ownership change.
            unsafe { raylib::ffi::GenTextureMipmaps(&mut font.as_raw_mut().texture) };
            font.texture()
                .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
            Some(font)
        }
        Err(error) => {
            eprintln!("warning: could not load embedded {label} font: {error:?}");
            None
        }
    }
}

fn load_fighter_atlas_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    character: CharacterId,
    baseline_path: &str,
) -> Option<SpriteAtlasAsset> {
    if reviewed_sprite_art_enabled() {
        let key = character.audio_key();
        let candidate_path = format!("assets/candidates/{key}/{key}-fighter.sprite.json");
        if asset_path(&candidate_path).is_file()
            && let Ok(combat_manifest) = SpriteManifest::load(asset_path(baseline_path))
            && let Some(mut candidate) = load_sprite_atlas_optional(raylib, thread, &candidate_path)
        {
            let missing = missing_candidate_clips(&candidate.manifest, character);
            if missing.is_empty() {
                candidate.combat_manifest = combat_manifest;
                return Some(candidate);
            }
            eprintln!(
                "warning: sprite candidate {candidate_path} is incomplete (missing {}); using {baseline_path}. Review partial candidates in Sprite Viewer.",
                missing.join(", ")
            );
        }
    }
    load_sprite_atlas_optional(raylib, thread, baseline_path)
}

fn reviewed_sprite_art_enabled() -> bool {
    match std::env::var("BORROW_FIGHTERS_SPRITE_CANDIDATES") {
        Ok(value) => value == "1",
        Err(std::env::VarError::NotPresent) => true,
        Err(std::env::VarError::NotUnicode(_)) => false,
    }
}

fn missing_candidate_clips(manifest: &SpriteManifest, character: CharacterId) -> Vec<&'static str> {
    FighterSpriteClip::required_for_character(character)
        .map(|clip| clip.as_str())
        .filter(|name| manifest.clip_named(name).is_none())
        .collect()
}

fn load_sprite_atlas_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    manifest_path: &str,
) -> Option<SpriteAtlasAsset> {
    let resolved_path = asset_path(manifest_path);
    let manifest = match SpriteManifest::load(&resolved_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("warning: could not load sprite manifest {manifest_path}: {error}");
            return None;
        }
    };
    let mut textures = Vec::new();
    for (image, path) in manifest.image_paths(&resolved_path) {
        let texture = load_texture_optional(raylib, thread, &path.to_string_lossy())?;
        textures.push(SpriteAtlasTexture { image, texture });
    }

    Some(SpriteAtlasAsset {
        combat_manifest: manifest.clone(),
        manifest,
        textures,
    })
}

fn load_projectile_texture_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    character: CharacterId,
    baseline_path: &str,
) -> Option<Texture2D> {
    if reviewed_sprite_art_enabled() {
        let key = character.audio_key();
        let candidate_path = format!("assets/candidates/{key}/{key}-projectile.png");
        if asset_path(&candidate_path).is_file()
            && let Some(texture) = load_texture_optional(raylib, thread, &candidate_path)
        {
            return Some(texture);
        }
    }
    load_texture_optional(raylib, thread, baseline_path)
}

fn load_smooth_texture_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
) -> Option<Texture2D> {
    let mut texture = load_texture_optional(raylib, thread, path)?;
    texture.gen_texture_mipmaps();
    texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
    Some(texture)
}

fn load_texture_optional(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
) -> Option<Texture2D> {
    match raylib.load_texture(thread, &asset_path(path).to_string_lossy()) {
        Ok(texture) => Some(texture),
        Err(error) => {
            eprintln!("warning: could not load texture {path}: {error:?}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::sprites::SpriteClip;

    #[test]
    fn partial_candidates_cannot_replace_a_complete_fighter_in_matches() {
        let mut manifest = SpriteManifest::load(RUST_FIGHTER_MANIFEST_PATH).unwrap();
        assert_eq!(
            missing_candidate_clips(&manifest, CharacterId::Rust),
            vec![
                "spawn",
                "crouch_block",
                "victory",
                "defeat",
                "knockdown",
                "signature_special",
                "heavy_hit",
                "launched",
                "thrown"
            ]
        );
        for name in [
            "spawn",
            "crouch_block",
            "victory",
            "defeat",
            "knockdown",
            "signature_special",
            "heavy_hit",
            "launched",
            "thrown",
        ] {
            manifest.clips.push(SpriteClip {
                name: name.to_string(),
                r#loop: false,
                frames: manifest.clip_named("idle").unwrap().frames.clone(),
            });
        }
        assert!(missing_candidate_clips(&manifest, CharacterId::Rust).is_empty());
        // Existing fallback artwork is not sufficient coverage for a candidate.
        manifest.clips.retain(|clip| clip.name != "throw");
        assert_eq!(
            missing_candidate_clips(&manifest, CharacterId::Rust),
            vec!["throw"]
        );
    }

    #[test]
    fn go_keeps_its_previous_coverage_while_the_five_require_new_actions() {
        let manifest = SpriteManifest::load("assets/candidates/go/go-fighter.sprite.json").unwrap();
        assert!(missing_candidate_clips(&manifest, CharacterId::Go).is_empty());
        for character in [
            CharacterId::Rust,
            CharacterId::Duke,
            CharacterId::C,
            CharacterId::Python,
            CharacterId::Cpp,
        ] {
            assert_eq!(
                missing_candidate_clips(&manifest, character),
                vec![
                    "knockdown",
                    "signature_special",
                    "heavy_hit",
                    "launched",
                    "thrown"
                ]
            );
        }
    }
}
