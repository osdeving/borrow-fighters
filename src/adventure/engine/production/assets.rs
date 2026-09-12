//! Validates complete content packages before allocating their contextual textures.
//!
//! System: Adventure production loading. External character previews use exactly
//! the same loader as the campaign and never request Rust or opening resources.

use crate::adventure::{
    augusta::{ChapterSpec, Texts, World},
    production::{
        CharacterPack, CharacterSpec, CombatCatalog,
        animation::{Attachment, Clips, Rig, safe_relative},
    },
};
use crate::runtime_paths::asset_path;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Validated source artifact, available without a window or graphics device.
#[derive(Clone, Debug)]
pub struct ActorContent {
    pub pack: CharacterPack,
    pub rig: Rig,
    pub clips: Clips,
    pub root: PathBuf,
    pub image_extents: BTreeMap<String, [u32; 2]>,
}

fn text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// Resolves a contained relative resource, including a symlink containment check.
fn contained(root: &Path, value: &str) -> Result<PathBuf, String> {
    if !safe_relative(value) {
        return Err(format!("invalid relative content path {value}"));
    }
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let path = root
        .join(value)
        .canonicalize()
        .map_err(|e| format!("{value}: {e}"))?;
    if !path.starts_with(&root) {
        return Err(format!("resource escapes content package: {value}"));
    }
    Ok(path)
}

fn png_size(path: &Path) -> Result<[u32; 2], String> {
    use std::io::Read;
    let mut bytes = [0; 24];
    fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut bytes))
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if &bytes[..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return Err(format!("expected PNG texture {}", path.display()));
    }
    let size = [
        u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
        u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
    ];
    if size.iter().any(|v| *v == 0 || *v > 8192) {
        return Err("texture dimensions exceed 8192px package limit".into());
    }
    // Decode on the CPU before reporting a package valid. An intact IHDR can
    // precede truncated/corrupt image data; no window or GPU allocation is needed.
    let decoded = Image::load_image(&path.to_string_lossy())
        .map_err(|error| format!("{}: PNG decode failed: {error}", path.display()))?;
    if decoded.width as u32 != size[0] || decoded.height as u32 != size[1] {
        return Err(format!(
            "{}: decoded PNG dimensions disagree with IHDR",
            path.display()
        ));
    }
    Ok(size)
}

fn image_extents(
    root: &Path,
    attachments: &BTreeMap<String, Attachment>,
) -> Result<BTreeMap<String, [u32; 2]>, String> {
    let mut images = BTreeMap::new();
    for (id, a) in attachments {
        let size = if let Some(size) = images.get(&a.image) {
            *size
        } else {
            let size = png_size(&contained(root, &a.image)?)?;
            images.insert(a.image.clone(), size);
            size
        };
        if a.source[0] + a.source[2] > size[0] as f32 || a.source[1] + a.source[3] > size[1] as f32
        {
            return Err(format!("attachment {id} extends outside {}", a.image));
        }
    }
    Ok(images)
}

impl ActorContent {
    pub fn load(path: &Path) -> Result<Self, String> {
        let parent = path.parent().ok_or("character path has no parent")?;
        let root = if parent.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            parent.to_path_buf()
        };
        let source = text(path)?;
        let spec: CharacterSpec = serde_json::from_str(&source).map_err(|e| e.to_string())?;
        let pack = CharacterPack::from_json(&source, &text(&contained(&root, &spec.combat)?)?)?;
        let rig = Rig::from_json(&text(&contained(&root, &spec.rig)?)?)?;
        if rig.character_id != pack.character.id {
            return Err("character and rig identities disagree".into());
        }
        let clips = Clips::from_json(&text(&contained(&root, &spec.clips)?)?, &rig)?;
        for id in [
            "idle", "start", "run", "stop", "turn", "jump", "fall", "land", "guard", "parry",
            "hurt", "knockout", "arrival", "interact",
        ] {
            if !clips.clips.contains_key(id) {
                return Err(format!("missing required clip {id} for {}", spec.id));
            }
        }
        for movement in &pack.combat.moves {
            if let Some(projectile) = &movement.projectile {
                let effect = rig
                    .effects
                    .get(&projectile.visual_id)
                    .ok_or_else(|| format!("missing projectile art {}", projectile.visual_id))?;
                effect.validate()?;
                if effect
                    .sprite
                    .as_ref()
                    .is_some_and(|id| !rig.attachments.contains_key(id))
                {
                    return Err(format!(
                        "missing projectile sprite {}",
                        projectile.visual_id
                    ));
                }
            }
            let clip = clips
                .clips
                .get(&movement.animation_id)
                .ok_or_else(|| format!("missing move clip {}", movement.animation_id))?;
            if clip.duration_ticks != movement.duration() || clip.looping {
                return Err(format!(
                    "clip {} must last {} ticks without looping",
                    movement.animation_id,
                    movement.duration()
                ));
            }
        }
        let image_extents = image_extents(&root, &rig.attachments)?;
        Ok(Self {
            pack,
            rig,
            clips,
            root,
            image_extents,
        })
    }
}

/// One character's resources; each image path is uploaded once per package.
pub struct ActorAssets {
    pub pack: CharacterPack,
    pub rig: Rig,
    pub clips: Clips,
    pub textures: BTreeMap<String, Texture2D>,
}

impl ActorAssets {
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        Self::from_content(
            rl,
            thread,
            ActorContent::load(path).map_err(|s| -> Box<dyn Error> { s.into() })?,
        )
    }
    pub fn from_content(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        c: ActorContent,
    ) -> Result<Self, Box<dyn Error>> {
        let textures = load_textures(rl, thread, &c.root, &c.image_extents)?;
        Ok(Self {
            pack: c.pack,
            rig: c.rig,
            clips: c.clips,
            textures,
        })
    }
}

fn load_textures(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    root: &Path,
    images: &BTreeMap<String, [u32; 2]>,
) -> Result<BTreeMap<String, Texture2D>, Box<dyn Error>> {
    images
        .keys()
        .map(|name| {
            let mut texture = rl.load_texture(thread, &root.join(name).to_string_lossy())?;
            texture.gen_texture_mipmaps();
            texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
            texture.set_texture_wrap(thread, TextureWrap::TEXTURE_WRAP_CLAMP);
            Ok((name.clone(), texture))
        })
        .collect()
}

/// Editor-style scene export. Actor links are relative to the adventure asset root.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldArt {
    pub version: u32,
    pub actors: BTreeMap<String, String>,
    pub pieces: BTreeMap<String, Attachment>,
    pub background: String,
    pub ground: String,
    /// Mirror alternating floor modules when their painted edges are not periodic.
    #[serde(default)]
    pub mirror_ground_tiles: bool,
    pub parallax: f32,
    pub facade_baseline: f32,
    pub signs: Vec<Sign>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sign {
    pub text: String,
    pub position: [f32; 2],
    pub size: f32,
    pub color: [u8; 3],
}

#[derive(Clone, Debug, Serialize)]
pub struct LoadReport {
    pub chapter: String,
    pub actors: Vec<String>,
    pub texture_count: usize,
    pub decoded_rgba_bytes: u64,
    pub resources: Vec<String>,
}

/// No opening, fighting roster, Rust rig or other chapter is loaded by this package.
pub struct ProductionAssets {
    pub spec: ChapterSpec,
    pub world: World,
    pub texts: Texts,
    pub catalog: Arc<CombatCatalog>,
    pub actors: BTreeMap<String, ActorAssets>,
    pub art: WorldArt,
    pub textures: BTreeMap<String, Texture2D>,
    pub font: Font,
    pub report: LoadReport,
    pub animations: RefCell<BTreeMap<u32, crate::adventure::production::animation::Animator>>,
}

impl ProductionAssets {
    /// Advances presentation transitions on the same fixed ticks as gameplay.
    pub fn advance_animations(&self, simulation: &crate::adventure::production::Simulation) {
        let mut states = self.animations.borrow_mut();
        states.retain(|id, _| simulation.actors().iter().any(|a| a.id.0 == *id));
        for actor in simulation.actors() {
            let clips = &self.actors[&actor.character].clips;
            let state = states
                .entry(actor.id.0)
                .or_insert_with(|| crate::adventure::production::animation::Animator::new(clips));
            state.tick(
                clips,
                actor.clip_id(),
                actor.action_ticks as f32,
                actor.stride_distance,
            );
        }
    }
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        let spec = ChapterSpec::load(path)?;
        let root = path.parent().ok_or("chapter path has no parent")?;
        let world = World::load(&contained(root, &spec.world)?)?;
        let texts = Texts::load(&contained(root, &spec.texts)?)?;
        let art: WorldArt = serde_json::from_str(&text(&contained(root, &spec.art)?)?)?;
        if art.version != 1
            || !(0.0..=1.0).contains(&art.parallax)
            || !art.facade_baseline.is_finite()
            || !art.pieces.contains_key(&art.background)
            || !art.pieces.contains_key(&art.ground)
            || art.signs.iter().any(|s| {
                s.text.is_empty()
                    || !s.size.is_finite()
                    || s.size <= 0.0
                    || s.position.iter().any(|x| !x.is_finite())
            })
        {
            return Err("invalid world art descriptor".into());
        }
        // Reuse the same structural attachment validation as standalone actors.
        Rig {
            schema_version: 1,
            character_id: "world".into(),
            method: crate::adventure::production::animation::Method::Frames,
            height: 100.0,
            attachments: art.pieces.clone(),
            skeleton: None,
            views: vec![],
            effects: BTreeMap::new(),
        }
        .validate()?;
        for piece in &world.pieces {
            if !art.pieces.contains_key(&piece.piece) {
                return Err(format!("missing world piece {}", piece.piece).into());
            }
        }
        let mut candidates = BTreeMap::new();
        let actor_root = asset_path("assets/adventure");
        for (id, link) in &art.actors {
            let c = ActorContent::load(&contained(&actor_root, link)?)?;
            if &c.pack.character.id != id {
                return Err(format!("actor entry {id} has a different identity").into());
            }
            candidates.insert(id.clone(), c);
        }
        for id in [&spec.protagonist, "security", "erratic", "julia", "broker"] {
            if !candidates.contains_key(id) {
                return Err(format!("chapter requires actor {id}").into());
            }
        }
        let catalog = Arc::new(CombatCatalog::new(
            candidates.values().map(|c| c.pack.clone()).collect(),
        )?);
        let extents = image_extents(root, &art.pieces)?;
        let mut report = LoadReport {
            chapter: spec.id.clone(),
            actors: candidates.keys().cloned().collect(),
            texture_count: 0,
            decoded_rgba_bytes: 0,
            resources: vec![],
        };
        for (base, images) in candidates
            .values()
            .map(|c| (&*c.root, &c.image_extents))
            .chain(std::iter::once((root, &extents)))
        {
            for (path, size) in images {
                report.resources.push(base.join(path).display().to_string());
                report.texture_count += 1;
                report.decoded_rgba_bytes += u64::from(size[0]) * u64::from(size[1]) * 4;
            }
        }
        // All CPU-side references were accepted before any scene texture is replaced.
        let mut actors = BTreeMap::new();
        for (id, c) in candidates {
            actors.insert(id, ActorAssets::from_content(rl, thread, c)?);
        }
        let textures = load_textures(rl, thread, root, &extents)?;
        let glyphs: String = (32..=591)
            .filter_map(char::from_u32)
            .chain("—–…←→".chars())
            .collect();
        let font = rl.load_font_ex(
            thread,
            &asset_path("assets/adventure/fonts/Barlow-Regular.ttf").to_string_lossy(),
            48,
            Some(&glyphs),
        )?;
        Ok(Self {
            spec,
            world,
            texts,
            catalog,
            actors,
            art,
            textures,
            font,
            report,
            animations: RefCell::new(BTreeMap::new()),
        })
    }
}
