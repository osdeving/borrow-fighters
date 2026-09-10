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
    /// Named attachment points in this frame's source rectangle, such as a hand.
    #[serde(default)]
    pub sockets: BTreeMap<String, [f32; 2]>,
    /// Optional authored hold; absent frames use the clip's default duration.
    #[serde(default)]
    pub duration_ticks: Option<u32>,
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
    /// Loop ordinary movement; narrative actions can hold their final pose.
    #[serde(default = "default_looping")]
    pub looping: bool,
}

fn default_looping() -> bool {
    true
}

impl Piece {
    /// Chooses a frame and uniform scale without tying it to atlas dimensions.
    pub fn sample(&self, ticks: u32) -> (&PieceFrame, f32) {
        let duration = self.duration();
        let mut cursor = if self.looping {
            ticks % duration
        } else {
            ticks.min(duration - 1)
        };
        let frame = self
            .frames
            .iter()
            .find(|frame| {
                let hold = frame.duration_ticks.unwrap_or(self.frame_ticks);
                if cursor < hold {
                    true
                } else {
                    cursor -= hold;
                    false
                }
            })
            .expect("validated clips have positive duration");
        (frame, self.width / self.frames[0].source[2])
    }

    /// Total fixed duration of the clip before looping or holding its last pose.
    pub fn duration(&self) -> u32 {
        self.frames
            .iter()
            .map(|frame| frame.duration_ticks.unwrap_or(self.frame_ticks))
            .sum()
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
                for (name, [sx, sy]) in &frame.sockets {
                    if name.trim().is_empty()
                        || !sx.is_finite()
                        || !sy.is_finite()
                        || !(0.0..=w).contains(sx)
                        || !(0.0..=h).contains(sy)
                    {
                        return Err(format!("invalid attachment socket in {id}").into());
                    }
                }
                if frame.duration_ticks == Some(0) {
                    return Err(format!("zero frame duration in {id}").into());
                }
            }
            if piece
                .frames
                .iter()
                .try_fold(0u32, |sum, frame| {
                    sum.checked_add(frame.duration_ticks.unwrap_or(piece.frame_ticks))
                })
                .is_none()
            {
                return Err(format!("clip duration overflow in {id}").into());
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
    /// Empty composition for catalogs whose actors are placed by chapter geometry.
    pub fn empty() -> Self {
        Self {
            version: 1,
            props: Vec::new(),
        }
    }
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

    fn timed_catalog() -> PieceCatalog {
        let mut catalog = catalog();
        let piece = catalog.pieces.get_mut("test").unwrap();
        let mut drawing = piece.frames[0].clone();
        drawing.image = "chapter/draw.png".into();
        drawing.duration_ticks = Some(2);
        let mut reading = piece.frames[0].clone();
        reading.image = "chapter/read.png".into();
        reading.source[2] = 200.0;
        let mut stowing = piece.frames[0].clone();
        stowing.image = "chapter/stow.png".into();
        stowing.source[2] = 600.0;
        stowing.duration_ticks = Some(4);
        piece.frames = vec![drawing, reading, stowing];
        catalog.validate().unwrap();
        catalog
    }

    #[test]
    fn legacy_uniform_clips_still_loop_without_new_optional_fields() {
        let mut catalog = catalog();
        let piece = catalog.pieces.get_mut("test").unwrap();
        let mut second = piece.frames[0].clone();
        second.image = "street/second.png".into();
        piece.frames.push(second);
        catalog.validate().unwrap();
        let piece = &catalog.pieces["test"];
        assert!(piece.looping);
        assert_eq!(piece.duration(), 12);
        for (ticks, expected) in [(0, 0), (5, 0), (6, 1), (11, 1), (12, 0)] {
            let frame = piece.sample(ticks).0;
            assert!(std::ptr::eq(frame, &piece.frames[expected]), "tick {ticks}");
            assert!(frame.sockets.is_empty());
        }
    }

    #[test]
    fn variable_holds_switch_at_exact_boundaries_and_share_the_first_frame_scale() {
        let catalog = timed_catalog();
        let piece = &catalog.pieces["test"];
        assert_eq!(piece.duration(), 12);
        for (ticks, expected) in [
            (0, 0),
            (1, 0),
            (2, 1),
            (7, 1),
            (8, 2),
            (11, 2),
            (12, 0),
            (13, 0),
            (14, 1),
            (23, 2),
            (24, 0),
            (u32::MAX, 1),
        ] {
            let (frame, scale) = piece.sample(ticks);
            assert!(std::ptr::eq(frame, &piece.frames[expected]), "tick {ticks}");
            assert_eq!(scale, 0.4, "pose changes must not rescale the actor");
        }
    }

    #[test]
    fn nonlooping_gestures_hold_the_last_pose_after_their_authored_duration() {
        let mut catalog = timed_catalog();
        catalog.pieces.get_mut("test").unwrap().looping = false;
        catalog.validate().unwrap();
        let piece = &catalog.pieces["test"];
        for (ticks, expected) in [(0, 0), (1, 0), (2, 1), (7, 1)] {
            assert!(std::ptr::eq(piece.sample(ticks).0, &piece.frames[expected]));
        }
        for ticks in [8, 11, 12, 13, 120, u32::MAX] {
            assert!(std::ptr::eq(piece.sample(ticks).0, &piece.frames[2]));
        }
    }

    #[test]
    fn clip_durations_reject_zero_and_overflow_but_allow_the_largest_valid_total() {
        let mut zero = catalog();
        zero.pieces.get_mut("test").unwrap().frames[0].duration_ticks = Some(0);
        assert_eq!(
            zero.validate().unwrap_err().to_string(),
            "zero frame duration in test"
        );

        let mut catalog = timed_catalog();
        let piece = catalog.pieces.get_mut("test").unwrap();
        piece.frames.truncate(2);
        piece.frames[0].duration_ticks = Some(u32::MAX);
        piece.frames[1].duration_ticks = Some(1);
        assert_eq!(
            catalog.validate().unwrap_err().to_string(),
            "clip duration overflow in test"
        );

        let piece = catalog.pieces.get_mut("test").unwrap();
        piece.frames[0].duration_ticks = Some(u32::MAX - 1);
        piece.looping = false;
        catalog.validate().unwrap();
        let piece = &catalog.pieces["test"];
        assert_eq!(piece.duration(), u32::MAX);
        assert!(std::ptr::eq(piece.sample(u32::MAX - 2).0, &piece.frames[0]));
        assert!(std::ptr::eq(piece.sample(u32::MAX - 1).0, &piece.frames[1]));
        assert!(std::ptr::eq(piece.sample(u32::MAX).0, &piece.frames[1]));
    }

    #[test]
    fn sockets_accept_source_relative_edges_and_reject_invalid_names_or_coordinates() {
        let mut valid = catalog();
        let sockets = &mut valid.pieces.get_mut("test").unwrap().frames[0].sockets;
        sockets.insert("phone".into(), [0.0, 0.0]);
        sockets.insert("phone_tip".into(), [400.0, 200.0]);
        valid.validate().unwrap();
        let frame = valid.pieces["test"].sample(0).0;
        assert_eq!(frame.sockets["phone"], [0.0, 0.0]);
        assert_eq!(frame.sockets["phone_tip"], [400.0, 200.0]);

        for (name, point) in [
            ("", [0.0, 0.0]),
            (" \t", [0.0, 0.0]),
            ("phone", [-0.01, 20.0]),
            ("phone", [400.01, 20.0]),
            ("phone", [20.0, -0.01]),
            ("phone", [20.0, 200.01]),
            ("phone", [f32::NAN, 20.0]),
            ("phone", [20.0, f32::INFINITY]),
            ("phone", [f32::NEG_INFINITY, 20.0]),
        ] {
            let mut invalid = catalog();
            invalid.pieces.get_mut("test").unwrap().frames[0]
                .sockets
                .insert(name.into(), point);
            assert_eq!(
                invalid.validate().unwrap_err().to_string(),
                "invalid attachment socket in test",
                "socket {name:?} at {point:?}"
            );
        }
    }

    #[test]
    fn chapter_catalog_loads_and_every_frame_resolves_inside_its_local_png() {
        use std::io::Read;

        let base = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/adventure")
            .canonicalize()
            .unwrap();
        let catalog = PieceCatalog::load(&base.join("chapter/catalog.json")).unwrap();
        StreetLayout::empty().validate(&catalog).unwrap();
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
        ] {
            assert!(catalog.pieces.contains_key(id), "missing chapter clip {id}");
        }
        for (id, piece) in &catalog.pieces {
            for frame in &piece.frames {
                let path = base.join(&frame.image).canonicalize().unwrap();
                assert!(path.starts_with(&base), "image escapes adventure: {id}");
                let mut header = [0; 24];
                fs::File::open(&path)
                    .unwrap()
                    .read_exact(&mut header)
                    .unwrap();
                assert_eq!(&header[..8], b"\x89PNG\r\n\x1a\n", "{id}: {path:?}");
                assert_eq!(&header[12..16], b"IHDR", "{id}: {path:?}");
                let width = u32::from_be_bytes(header[16..20].try_into().unwrap());
                let height = u32::from_be_bytes(header[20..24].try_into().unwrap());
                assert!(frame.source[0] + frame.source[2] <= width as f32, "{id}");
                assert!(frame.source[1] + frame.source[3] <= height as f32, "{id}");
            }
        }
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
