//! Defines replaceable visual pieces and the street's independent composition.
//!
//! System: Adventure content. Catalog frames own image geometry; scene instances
//! only choose pieces and placements. Neither file defines physical actors.

use serde::Deserialize;
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Component, Path},
};

/// A frame whose image can be a standalone PNG or a region of an atlas.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceFrame {
    /// Asset path relative to assets/adventure, without parent traversal.
    pub image: String,
    /// Pixel rectangle in the source image.
    pub source: [f32; 4],
    /// Pixel anchor relative to the source rectangle, usually feet or wheel ground.
    pub anchor: [f32; 2],
    /// Optional wheel centers and glint radius relative to the source rectangle.
    #[serde(default)]
    pub wheels: Vec<[f32; 3]>,
}

/// Authored frames at one stable world scale, independent of scene placement.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Piece {
    /// Display width of the first frame; later frames retain its pixel scale.
    pub width: f32,
    /// Fixed updates per frame, shared by every instance of this piece.
    pub frame_ticks: u32,
    /// Ordered frames, each allowed to reference a different source image.
    pub frames: Vec<PieceFrame>,
}

impl Piece {
    /// Chooses a frame and uniform scale without tying it to atlas dimensions.
    pub fn sample(&self, ticks: u32) -> (&PieceFrame, f32) {
        (
            &self.frames[(ticks / self.frame_ticks) as usize % self.frames.len()],
            self.width / self.frames[0].source[2],
        )
    }
}

/// Named art pieces reusable by many scene instances.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceCatalog {
    version: u32,
    /// IDs retain their meaning when paths, rectangles or frames are replaced.
    pub pieces: BTreeMap<String, Piece>,
}

impl PieceCatalog {
    /// Reads and validates content before the renderer loads its textures.
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let catalog: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        catalog.validate()?;
        Ok(catalog)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.version != 1 || self.pieces.is_empty() {
            return Err("piece catalog requires version 1 and at least one piece".into());
        }
        for (id, piece) in &self.pieces {
            if id.trim().is_empty()
                || !piece.width.is_finite()
                || piece.width <= 0.0
                || piece.width > 1280.0
                || piece.frame_ticks == 0
                || piece.frames.is_empty()
            {
                return Err(format!("invalid piece: {id}").into());
            }
            for frame in &piece.frames {
                let path = Path::new(&frame.image);
                let [x, y, w, h] = frame.source;
                if path.extension().is_none_or(|ext| ext != "png")
                    || frame.image.contains(['\\', ':'])
                    || frame
                        .image
                        .split('/')
                        .any(|part| matches!(part, "" | "." | ".."))
                    || !path
                        .components()
                        .all(|part| matches!(part, Component::Normal(_)))
                    || !frame
                        .source
                        .iter()
                        .chain(frame.anchor.iter())
                        .all(|v| v.is_finite())
                    || x < 0.0
                    || y < 0.0
                    || w <= 0.0
                    || h <= 0.0
                    || !(0.0..=w).contains(&frame.anchor[0])
                    || !(0.0..=h).contains(&frame.anchor[1])
                {
                    return Err(format!("invalid image, rectangle or anchor in {id}").into());
                }
                for [wx, wy, radius] in &frame.wheels {
                    if ![wx, wy, radius].iter().all(|v| v.is_finite())
                        || !(0.0..=w).contains(wx)
                        || !(0.0..=h).contains(wy)
                        || *radius <= 0.0
                    {
                        return Err(format!("invalid wheel landmark in {id}").into());
                    }
                }
            }
        }
        Ok(())
    }
}

/// Editable copy attached to a prop instead of painted into its bitmap.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropLabel {
    /// Key in the existing Portuguese text catalog.
    pub text_key: String,
    /// Offset from the prop's placement, in world pixels.
    pub offset: [f32; 2],
    /// Available line width before text shrinks to fit.
    pub width: f32,
    /// Maximum font size.
    pub font_size: f32,
    /// Text tint, including alpha.
    pub color: [u8; 4],
}

/// One placement of a reusable decorative piece.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropInstance {
    /// Unique instance name; multiple instances may use the same piece.
    pub id: String,
    /// Reference into PieceCatalog.
    pub piece: String,
    /// Ground/anchor position before background camera translation.
    pub position: [f32; 2],
    /// Instance multiplier; source-independent authored size stays in the piece.
    pub scale: f32,
    /// Optional labels move with this prop.
    #[serde(default)]
    pub labels: Vec<PropLabel>,
}

/// Back-to-front prop composition, separate from texture geometry and motion.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreetLayout {
    version: u32,
    /// Ordered prop placements behind moving street actors.
    pub props: Vec<PropInstance>,
}

impl StreetLayout {
    /// Loads placements, rejecting broken references without silently omitting art.
    pub fn load(path: &Path, catalog: &PieceCatalog) -> Result<Self, Box<dyn Error>> {
        let layout: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        layout.validate(catalog)?;
        Ok(layout)
    }

    fn validate(&self, catalog: &PieceCatalog) -> Result<(), Box<dyn Error>> {
        if self.version != 1 {
            return Err("street layout requires version 1".into());
        }
        let mut ids = std::collections::BTreeSet::new();
        for prop in &self.props {
            if prop.id.trim().is_empty()
                || !ids.insert(&prop.id)
                || !catalog.pieces.contains_key(&prop.piece)
                || !prop.position.iter().all(|v| v.is_finite())
                || !prop.scale.is_finite()
                || prop.scale <= 0.0
                || prop.scale > 4.0
            {
                return Err(format!("invalid prop or piece reference: {}", prop.id).into());
            }
            for label in &prop.labels {
                if label.text_key.trim().is_empty()
                    || !label.offset.iter().all(|v| v.is_finite())
                    || !label.width.is_finite()
                    || label.width <= 0.0
                    || !label.font_size.is_finite()
                    || label.font_size <= 0.0
                {
                    return Err(format!("invalid prop label: {}", prop.id).into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> PieceCatalog {
        serde_json::from_value(serde_json::json!({"version":1,"pieces":{"test":{
            "width":160,"frame_ticks":6,"frames":[{"image":"street/car.png", "source":[32,64,400,200],"anchor":[200,200]}]
        }}})).unwrap()
    }

    #[test]
    fn art_can_change_source_and_resolution_without_changing_the_layout() {
        let mut catalog = catalog();
        let layout: StreetLayout = serde_json::from_value(serde_json::json!({"version":1,"props":[
            {"id":"first","piece":"test","position":[100,350],"scale":1},
            {"id":"second","piece":"test","position":[650,350],"scale":0.8}
        ]}))
        .unwrap();
        layout.validate(&catalog).unwrap();
        let piece = catalog.pieces.get_mut("test").unwrap();
        piece.frames[0].image = "replacement.png".into();
        piece.frames[0].source = [0.0, 0.0, 800.0, 400.0];
        piece.frames[0].anchor = [400.0, 400.0];
        catalog.validate().unwrap();
        layout.validate(&catalog).unwrap();
        assert_eq!(catalog.pieces["test"].sample(0).1, 0.2);
        assert_eq!(layout.props[0].position, [100.0, 350.0]);
        assert_eq!(layout.props[1].piece, "test");
    }

    #[test]
    fn bad_paths_geometry_and_missing_piece_references_are_rejected() {
        for image in [
            "../outside.png",
            "/absolute.png",
            "texture.jpg",
            "C:/car.png",
            "..\\car.png",
            "street//car.png",
            "street/./car.png",
        ] {
            let mut c = catalog();
            c.pieces.get_mut("test").unwrap().frames[0].image = image.into();
            assert!(c.validate().is_err());
        }
        let mut c = catalog();
        c.pieces.get_mut("test").unwrap().frames[0].anchor[0] = 401.0;
        assert!(c.validate().is_err());
        let layout: StreetLayout = serde_json::from_value(serde_json::json!({"version":1,"props":[
            {"id":"bad","piece":"absent","position":[0,0],"scale":1}
        ]}))
        .unwrap();
        assert!(layout.validate(&catalog()).is_err());
    }
}
