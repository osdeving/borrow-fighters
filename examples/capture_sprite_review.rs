//! Captures deterministic candidate sprite review images through the real Combat Lab renderer.
//!
//! System: Development examples at the Raylib boundary. Advances the existing
//! lab at 60 Hz, without keyboard automation or changes to combat rules, and
//! records which loaded candidate frame each image represents.

use std::{collections::BTreeSet, error::Error, fs, path::Path};

use borrow_fighters::{
    characters::CharacterId,
    config::{FIXED_TIMESTEP, WINDOW_HEIGHT, WINDOW_WIDTH},
    engine::{
        assets::{GameAssets, SpriteAtlasAsset},
        render::{draw_combat_lab, draw_render_target_to_window},
        sprites::{
            FighterSpriteClip, SpriteManifest, fighter_clip_elapsed_seconds, fighter_sprite_clip,
            frame_for_fighter_clip_at,
        },
    },
    scenes::combat_lab::{
        CombatLab, CombatLabInput, CombatLabMove, CombatLabOptions, CombatLabPose,
    },
};
use raylib::prelude::*;
use serde_json::{Value, json};

type CaptureResult<T> = Result<T, Box<dyn Error>>;

fn main() -> CaptureResult<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !(2..=3).contains(&args.len()) {
        return Err("usage: BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run --example capture_sprite_review -- <character> <output-directory> [clip,clip,...]".into());
    }
    if std::env::var("BORROW_FIGHTERS_SPRITE_CANDIDATES").as_deref() != Ok("1") {
        return Err("candidate opt-in is required: set BORROW_FIGHTERS_SPRITE_CANDIDATES=1".into());
    }
    let character = CharacterId::from_cli(&args[0]).ok_or("unknown character")?;
    let filter = capture_filter(args.get(2).map(String::as_str))?;
    let key = character.audio_key();
    let candidate_path = format!("assets/candidates/{key}/{key}-fighter.sprite.json");
    let expected = SpriteManifest::load(&candidate_path)?;
    validate_coverage(&expected)?;

    let (mut raylib, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Borrow Fighters - deterministic candidate review")
        .build();
    raylib.set_exit_key(None);
    let assets = GameAssets::load(&mut raylib, &thread);
    let atlas = selected_atlas(&assets, character).ok_or("selected fighter atlas did not load")?;
    let loaded_json = serde_json::to_value(&atlas.manifest)?;
    if loaded_json != serde_json::to_value(&expected)? {
        return Err(
            "runtime loaded a different manifest or baseline fallback; refusing candidate captures"
                .into(),
        );
    }
    for frame in &atlas.manifest.frames {
        let texture = atlas
            .texture_for_frame(frame)
            .ok_or("candidate frame texture did not load")?;
        let rect = frame.frame;
        if rect.x < 0
            || rect.y < 0
            || i64::from(rect.x) + i64::from(rect.w) > i64::from(texture.width())
            || i64::from(rect.y) + i64::from(rect.h) > i64::from(texture.height())
        {
            return Err(format!(
                "candidate frame {} exceeds loaded texture bounds",
                frame.name
            )
            .into());
        }
    }

    let output = Path::new(&args[1]);
    fs::create_dir_all(output)?;
    let mut target =
        raylib.load_render_texture(&thread, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;
    let mut captures = Vec::new();
    for selected_move in CombatLabMove::ALL {
        if filter
            .as_ref()
            .is_some_and(|clips| !clips.contains(move_clip(selected_move).as_str()))
        {
            continue;
        }
        let options = CombatLabOptions {
            character,
            selected_move,
            pose: CombatLabPose::Move,
        };
        let mut lab = CombatLab::new(options);
        lab.set_combat_manifest(Some(atlas.combat_manifest.clone()));
        capture_sequence(
            &mut raylib,
            &thread,
            &mut target,
            &assets,
            &mut lab,
            move_clip(selected_move).as_str(),
            90,
            output,
            &mut captures,
        )?;
    }
    for pose in CombatLabPose::ALL
        .into_iter()
        .filter(|pose| *pose != CombatLabPose::Move)
    {
        if filter
            .as_ref()
            .is_some_and(|clips| !clips.contains(pose.label()))
        {
            continue;
        }
        let options = CombatLabOptions {
            character,
            pose,
            ..CombatLabOptions::default()
        };
        let mut lab = CombatLab::new(options);
        lab.set_combat_manifest(Some(atlas.combat_manifest.clone()));
        let clip = atlas
            .manifest
            .clip_named(pose.label())
            .ok_or("pose clip is missing")?;
        let duration_ms: u64 = clip
            .frames
            .iter()
            .filter_map(|name| atlas.manifest.frame_named(name))
            .map(|frame| u64::from(frame.duration_ms))
            .sum();
        let ticks = u16::try_from((duration_ms * 60).div_ceil(1000) + 6)
            .map_err(|_| "pose duration exceeds the lab frame counter")?;
        capture_sequence(
            &mut raylib,
            &thread,
            &mut target,
            &assets,
            &mut lab,
            pose.label(),
            ticks,
            output,
            &mut captures,
        )?;
    }
    let captured_clips: BTreeSet<_> = captures
        .iter()
        .filter_map(|capture| capture["clip"].as_str())
        .collect();
    let uncaptured_clips: Vec<_> = FighterSpriteClip::REQUIRED
        .iter()
        .map(|clip| clip.as_str())
        .filter(|clip| !captured_clips.contains(clip))
        .collect();
    let report = json!({
        "character": key, "candidate_manifest": candidate_path,
        "loaded_manifest_matches_candidate": true,
        "required_clips_checked": FighterSpriteClip::REQUIRED.iter().map(|clip| clip.as_str()).collect::<Vec<_>>(),
        "loaded_manifest": loaded_json,
        "loaded_combat_manifest": atlas.combat_manifest,
        "requested_clip_filter": filter,
        "fixed_timestep_seconds": FIXED_TIMESTEP,
        "render_size": [WINDOW_WIDTH, WINDOW_HEIGHT],
        "captured_clips": captured_clips, "uncaptured_clips": uncaptured_clips,
        "limitations": [
            "Images are review artifacts, not assertions of visual correctness.",
            "Static lab poses advance art time only; they do not simulate match damage, outcome transitions, or a full jump trajectory.",
            "Combat Lab has no walk pose; walking and match transitions require a separate playtest.",
            "The lab uses its native facing for this character; no fighter state is modified to mirror it.",
            "Advantage estimates and automatic dummy placement use MoveSpec; drawn boxes and projectile emission use baseline sprite metadata with match-equivalent fallback."
        ],
        "captures": captures,
    });
    fs::write(
        output.join("capture-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    println!(
        "captured {} PNGs for {key}; report: {}",
        report["captures"].as_array().unwrap().len(),
        output.join("capture-report.json").display()
    );
    Ok(())
}

fn capture_filter(value: Option<&str>) -> CaptureResult<Option<BTreeSet<String>>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let available: BTreeSet<_> = CombatLabMove::ALL
        .into_iter()
        .map(|selected| move_clip(selected).as_str())
        .chain(
            CombatLabPose::ALL
                .into_iter()
                .filter(|pose| *pose != CombatLabPose::Move)
                .map(CombatLabPose::label),
        )
        .collect();
    let mut requested = BTreeSet::new();
    for name in value.split(',').map(str::trim) {
        if !available.contains(name) {
            return Err(format!(
                "unknown or unavailable Lab clip {name:?}; walk requires the World motion capture"
            )
            .into());
        }
        requested.insert(name.to_string());
    }
    Ok(Some(requested))
}

fn validate_coverage(manifest: &SpriteManifest) -> CaptureResult<()> {
    for required in FighterSpriteClip::REQUIRED {
        let name = required.as_str();
        let clip = manifest
            .clip_named(name)
            .ok_or_else(|| format!("candidate is missing {name}"))?;
        for frame_name in &clip.frames {
            let frame = manifest
                .frame_named(frame_name)
                .ok_or("candidate frame is missing")?;
            if frame.clip != name {
                return Err(format!("{name} references frame {frame_name} from {}; refusing aliased candidate coverage", frame.clip).into());
            }
        }
    }
    Ok(())
}

fn selected_atlas(assets: &GameAssets, character: CharacterId) -> Option<&SpriteAtlasAsset> {
    match character {
        CharacterId::Rust => assets.rust_fighter.as_ref(),
        CharacterId::Duke => assets.duke_fighter.as_ref(),
        CharacterId::Go => assets.go_fighter.as_ref(),
        CharacterId::C => assets.c_fighter.as_ref(),
        CharacterId::Python => assets.python_fighter.as_ref(),
        CharacterId::Cpp => assets.cpp_fighter.as_ref(),
    }
}

fn move_clip(selected: CombatLabMove) -> FighterSpriteClip {
    match selected {
        CombatLabMove::LightPunch => FighterSpriteClip::PunchLight,
        CombatLabMove::HeavyPunch => FighterSpriteClip::PunchHeavy,
        CombatLabMove::Kick => FighterSpriteClip::Kick,
        CombatLabMove::Sweep => FighterSpriteClip::Sweep,
        CombatLabMove::Overhead => FighterSpriteClip::Overhead,
        CombatLabMove::AntiAir => FighterSpriteClip::AntiAir,
        CombatLabMove::AirPunch => FighterSpriteClip::AirPunch,
        CombatLabMove::AirKick => FighterSpriteClip::AirKick,
        CombatLabMove::Throw => FighterSpriteClip::Throw,
        CombatLabMove::Projectile => FighterSpriteClip::Special,
    }
}

#[allow(clippy::too_many_arguments)]
fn capture_sequence(
    raylib: &mut RaylibHandle,
    thread: &RaylibThread,
    target: &mut RenderTexture2D,
    assets: &GameAssets,
    lab: &mut CombatLab,
    action: &str,
    ticks: u16,
    output: &Path,
    captures: &mut Vec<Value>,
) -> CaptureResult<()> {
    let manifest = &selected_atlas(assets, lab.character())
        .ok_or("atlas missing")?
        .manifest;
    let mut previous_visual = None;
    let mut previous_phase = String::new();
    for tick in 0..=ticks {
        if raylib.window_should_close() {
            return Err("capture window closed before review completed".into());
        }
        let (clip, seconds) = if lab.pose() == CombatLabPose::Move {
            (
                fighter_sprite_clip(lab.fighter()),
                fighter_clip_elapsed_seconds(lab.fighter(), lab.elapsed_seconds()),
            )
        } else {
            let clip = FighterSpriteClip::REQUIRED
                .into_iter()
                .find(|clip| clip.as_str() == lab.pose().label())
                .ok_or("unknown pose clip")?;
            (clip, lab.elapsed_seconds())
        };
        let frame =
            frame_for_fighter_clip_at(manifest, clip, seconds).ok_or("selected frame missing")?;
        let phase = format!("{:?}", lab.fighter().attack_phase()).to_lowercase();
        let visual = (clip, frame.name.as_str());
        let scheduled = [0, 1, 6, 12, 18, 30, 45, 60, 90].contains(&tick) || tick == ticks;
        if scheduled || previous_visual != Some(visual) || previous_phase != phase {
            {
                let mut draw = raylib.begin_texture_mode(thread, target);
                draw_combat_lab(&mut draw, lab, assets);
            }
            {
                let mut draw = raylib.begin_drawing(thread);
                draw_render_target_to_window(&mut draw, target);
            }
            // Read the same offscreen target used by the app, avoiding window
            // compositor focus/swap timing. GPU render targets have inverted Y.
            let mut image = target.texture().load_image()?;
            image.flip_vertical();
            let filename = format!("{action}-f{tick:03}-{phase}.png");
            let bytes = image.export_image_to_memory(".png")?;
            fs::write(output.join(&filename), &*bytes)?;
            let body = lab.fighter().body_rect();
            captures.push(json!({
                "image": filename, "action": action, "pose": lab.pose().label(),
                "character": lab.character().audio_key(),
                "facing": format!("{:?}", lab.fighter().facing),
                "lab_tick": lab.current_frame().get(), "clip": clip.as_str(),
                "sprite_frame": frame.name, "sprite_clip_seconds": seconds,
                "attack_phase": phase, "has_active_hitbox": lab.fighter().active_hitbox().is_some(),
                "projectile_count": lab.projectiles().len(),
                "projectile_rects": lab.projectiles().iter().map(|projectile| {
                    let rect = projectile.rect();
                    [rect.x, rect.y, rect.width, rect.height]
                }).collect::<Vec<_>>(),
                "attack_boxes": lab.attack_boxes().iter().map(|rect| [rect.x, rect.y, rect.width, rect.height]).collect::<Vec<_>>(),
                "hurtboxes": lab.hurtboxes().iter().map(|rect| [rect.x, rect.y, rect.width, rect.height]).collect::<Vec<_>>(),
                "body_rect": [body.x, body.y, body.width, body.height],
                "forced_pose": lab.pose() != CombatLabPose::Move,
            }));
        }
        previous_visual = Some(visual);
        previous_phase = phase;
        if tick < ticks {
            lab.update(CombatLabInput::default());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_filter_accepts_mixed_moves_and_poses_and_rejects_uncapturable_names() {
        assert!(capture_filter(None).unwrap().is_none());
        let clips = capture_filter(Some("punch_light, special,defeat,special"))
            .unwrap()
            .unwrap();
        assert_eq!(
            clips.into_iter().collect::<Vec<_>>(),
            ["defeat", "punch_light", "special"]
        );
        for invalid in ["walk", "unknown", "", "special,"] {
            assert!(capture_filter(Some(invalid)).is_err());
        }
    }
}
