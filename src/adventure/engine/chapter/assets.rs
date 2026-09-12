//! Loads replaceable chapter art, phone styling and externally editable copy.
//!
//! System: Adventure chapter assets. Common adventure graphics stay reusable;
//! this extension owns only the new clips, background and messenger appearance.

use super::phone::PhoneSkin;
use crate::{
    adventure::{
        chapter::{ChapterTexts, World},
        engine::{assets::Assets, pieces::StreetPieces},
        text::TextCatalog,
    },
    runtime_paths::asset_path,
};
use raylib::prelude::*;
use std::{error::Error, fs};

/// Complete visual resources for one chapter session, released when it returns.
pub struct ChapterAssets {
    pub common: Assets,
    pub pieces: StreetPieces,
    pub skin: PhoneSkin,
    pub texts: ChapterTexts,
}

impl ChapterAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let common = Assets::load(
            rl,
            thread,
            TextCatalog::load(asset_path("assets/adventure/texts/pt-BR.json"))?,
        )?;
        let pieces =
            StreetPieces::load_catalog(rl, thread, "assets/adventure/chapter/catalog.json")?;
        validate_clips(&pieces.catalog)?;
        validate_world_pieces(&load_world()?, &pieces.catalog)?;
        Ok(Self {
            common,
            pieces,
            skin: load_skin()?,
            texts: load_texts()?,
        })
    }

    /// Replaces text and phone appearance together only after both files validate.
    pub fn reload_copy(&mut self) -> Result<(), Box<dyn Error>> {
        let texts = load_texts()?;
        let skin = load_skin()?;
        self.texts = texts;
        self.skin = skin;
        Ok(())
    }
}

fn load_texts() -> Result<ChapterTexts, Box<dyn Error>> {
    Ok(ChapterTexts::from_json(&fs::read_to_string(asset_path(
        "assets/adventure/chapter/chapter-texts.json",
    ))?)?)
}

fn load_skin() -> Result<PhoneSkin, Box<dyn Error>> {
    Ok(serde_json::from_str(&fs::read_to_string(asset_path(
        "assets/adventure/chapter/phone-style.json",
    ))?)?)
}

/// Loads the same explicit level geometry used by triggers, physics and drawing.
pub fn load_world() -> Result<World, Box<dyn Error>> {
    Ok(World::from_json(&fs::read_to_string(asset_path(
        "assets/adventure/chapter/world.json",
    ))?)?)
}

fn validate_world_pieces(
    world: &World,
    catalog: &crate::adventure::scenery::PieceCatalog,
) -> Result<(), Box<dyn Error>> {
    for scene in &world.scenes {
        for prop in &scene.loose_props {
            if !catalog.pieces.contains_key(&prop.piece) {
                return Err(format!(
                    "loose prop {} references missing piece {}",
                    prop.id, prop.piece
                )
                .into());
            }
        }
        for prop in &scene.debris {
            for id in [&prop.piece, &prop.fragment] {
                if !catalog.pieces.contains_key(id) {
                    return Err(
                        format!("destructible {} references missing piece {id}", prop.id).into(),
                    );
                }
            }
        }
    }
    Ok(())
}

fn validate_clips(catalog: &crate::adventure::scenery::PieceCatalog) -> Result<(), Box<dyn Error>> {
    for id in [
        "rust.inspect",
        "rust.talk",
        "rust.listen",
        "rust.phone.draw",
        "rust.phone.read",
        "rust.phone.type",
        "rust.phone.stow",
        "driver.talk",
        "driver.idle",
        "prop.crate",
    ] {
        if !catalog.pieces.contains_key(id) {
            return Err(format!("missing chapter clip: {id}").into());
        }
    }
    for tick in 0..crate::adventure::chapter::phone::PHONE_DONE_TICK {
        let view = crate::adventure::chapter::phone::PhoneView::at(tick).expect("authored clock");
        if !view.device_visible {
            continue;
        }
        let (id, local) = super::phone::body_clip(view);
        let (frame, _) = catalog.pieces[id].sample(local);
        match (frame.sockets.get("phone"), frame.sockets.get("phone_tip")) {
            (Some(grip), Some(tip)) if grip != tip => {}
            _ => return Err(format!(
                "{id}: visible phone requires distinct phone and phone_tip sockets at tick {local}"
            )
            .into()),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_visible_phone_pose_requires_a_resolvable_hand_and_orientation() {
        let mut catalog = crate::adventure::scenery::PieceCatalog::load(&asset_path(
            "assets/adventure/chapter/catalog.json",
        ))
        .unwrap();
        validate_clips(&catalog).unwrap();
        catalog.pieces.get_mut("rust.phone.read").unwrap().frames[0]
            .sockets
            .remove("phone");
        assert!(
            validate_clips(&catalog)
                .unwrap_err()
                .to_string()
                .contains("rust.phone.read")
        );
    }
}
