//! Verifies portable package loading and rejects broken cross-file references.
//!
//! Tests copy the complete shipped actor into an isolated directory and invoke
//! the headless loader used by --validate, without allocating graphics resources.

use super::ActorContent;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Package(PathBuf);
impl Package {
    fn copy() -> Self {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/adventure/actors/cpp/character.json");
        let content = ActorContent::load(&source).unwrap();
        let root = std::env::temp_dir().join(format!(
            "borrow-actor-package-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        fs::copy(&source, root.join("character.json")).unwrap();
        for relative in [
            &content.pack.character.combat,
            &content.pack.character.rig,
            &content.pack.character.clips,
        ]
        .into_iter()
        .chain(content.image_extents.keys())
        {
            let target = root.join(relative);
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::copy(content.root.join(relative), target).unwrap();
        }
        Self(root)
    }
    fn manifest(&self) -> PathBuf {
        self.0.join("character.json")
    }
    fn mutate(&self, name: &str, change: impl FnOnce(&mut serde_json::Value)) -> String {
        let path = self.0.join(name);
        let original = fs::read_to_string(&path).unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&original).unwrap();
        change(&mut value);
        fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
        original
    }
}
impl Drop for Package {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_custom_manifest_with_the_same_id_does_not_inherit_the_installed_glb() {
    let package = Package::copy();
    let installed =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/actors/cpp/character.json");
    let custom = ActorContent::load(&package.manifest()).unwrap();
    assert_eq!(custom.pack.character.id, "cpp");
    assert!(!super::registered_manifest(&package.manifest(), &installed));
    assert!(super::registered_manifest(&installed, &installed));
}

#[test]
fn explicit_model_catalog_belongs_to_the_custom_package() {
    let package = Package::copy();
    let catalog = package.0.join("humans.json");
    fs::write(&catalog, "{}").unwrap();
    let options = super::Options::parse([
        "lab".to_string(),
        "--actor".into(),
        package.manifest().display().to_string(),
        "--models".into(),
        catalog.display().to_string(),
    ])
    .unwrap();
    let id = "cpp".to_string();
    let (selected, actors) = super::model_selection(&options, &[(&options.actor, &id)]).unwrap();
    assert_eq!(selected, Some(catalog.canonicalize().unwrap()));
    assert!(actors.contains("cpp"));
}

#[test]
fn a_portable_actor_can_change_identity_without_lab_or_campaign_changes() {
    let package = Package::copy();
    package.mutate("character.json", |j| j["id"] = "external-actor".into());
    package.mutate("rig.json", |j| j["character_id"] = "external-actor".into());
    let content = ActorContent::load(&package.manifest()).unwrap();
    assert_eq!(content.pack.character.id, "external-actor");
    assert_eq!(content.rig.character_id, "external-actor");
    assert!(content.clips.clips.contains_key("spin"));
    assert!(!content.image_extents.is_empty());
    assert!(content.root.starts_with(&package.0));
}

#[test]
fn complete_loader_rejects_missing_images_bounds_and_move_clip_disagreement() {
    let package = Package::copy();
    let before = package.mutate("rig.json", |j| {
        let first = j["attachments"]
            .as_object_mut()
            .unwrap()
            .values_mut()
            .next()
            .unwrap();
        first["image"] = "sprites/missing-art.png".into();
    });
    assert!(
        ActorContent::load(&package.manifest())
            .unwrap_err()
            .contains("missing-art.png")
    );
    fs::write(package.0.join("rig.json"), &before).unwrap();
    package.mutate("rig.json", |j| {
        let first = j["attachments"]
            .as_object_mut()
            .unwrap()
            .values_mut()
            .next()
            .unwrap();
        first["source"][2] = 8192.into();
    });
    assert!(
        ActorContent::load(&package.manifest())
            .unwrap_err()
            .contains("outside")
    );
    fs::write(package.0.join("rig.json"), before).unwrap();
    let before = package.mutate("clips.json", |j| {
        j["clips"]["light-1"]["duration_ticks"] = 99.into()
    });
    assert!(
        ActorContent::load(&package.manifest())
            .unwrap_err()
            .contains("must last")
    );
    fs::write(package.0.join("clips.json"), before).unwrap();
    assert!(ActorContent::load(&package.manifest()).is_ok());
}

#[test]
fn package_rejects_bone_dimensions_that_would_invert_the_ik_clamp() {
    let package = Package::copy();
    for dimension in [
        "thigh",
        "shin",
        "upper_arm",
        "forearm",
        "ankle_height",
        "leg_width",
        "knee_blend",
    ] {
        let original = package.mutate("rig.json", |j| {
            j["skeleton"][dimension] = serde_json::json!(0.0001);
        });
        let error = ActorContent::load(&package.manifest()).unwrap_err();
        assert!(
            error.contains("skeleton dimensions"),
            "{dimension}: {error}"
        );
        fs::write(package.0.join("rig.json"), original).unwrap();
    }
    assert!(ActorContent::load(&package.manifest()).is_ok());
}

#[test]
fn package_rejects_nonfinite_and_out_of_range_optional_view_registration() {
    let package = Package::copy();
    for registration in ["hips", "shoulders", "bag"] {
        // JSON numbers can be finite as f64 but overflow the target f32 field.
        for value in [1e40_f64, 32001.0] {
            let original = package.mutate("rig.json", |j| {
                j["views"][0][registration] = if registration == "bag" {
                    serde_json::json!([value, 0.0])
                } else {
                    serde_json::json!([[value, 0.0], [0.0, 0.0]])
                };
            });
            let error = ActorContent::load(&package.manifest()).unwrap_err();
            assert!(error.contains("painted view"), "{registration}: {error}");
            fs::write(package.0.join("rig.json"), original).unwrap();
        }
    }
    assert!(ActorContent::load(&package.manifest()).is_ok());
}

#[test]
fn headless_validation_rejects_an_intact_png_header_without_pixel_data() {
    let package = Package::copy();
    let content = ActorContent::load(&package.manifest()).unwrap();
    let relative = content.image_extents.keys().next().unwrap();
    let path = package.0.join(relative);
    let source = fs::read(&path).unwrap();
    fs::write(&path, &source[..24]).unwrap();
    let error = ActorContent::load(&package.manifest()).unwrap_err();
    assert!(error.contains("PNG decode failed"), "{error}");
    fs::write(&path, source).unwrap();
    assert!(ActorContent::load(&package.manifest()).is_ok());
}

#[test]
fn native_review_approaches_before_melee_and_contacts_with_the_light_chain() {
    use crate::adventure::production::{
        Bounds, CharacterPack, CombatCatalog, EnemySpawn, Event, Facing, Simulation,
    };
    use std::sync::Arc;
    let pack = CharacterPack::from_json(
        include_str!("../../../assets/adventure/actors/cpp/character.json"),
        include_str!("../../../assets/adventure/actors/cpp/combat.json"),
    )
    .unwrap();
    let mut sim = Simulation::new(
        Arc::new(CombatCatalog::new(vec![pack]).unwrap()),
        Bounds {
            left: 0.0,
            right: 3000.0,
            floor_y: 565.0,
        },
        "cpp",
        500.0,
    )
    .unwrap();
    sim.begin_encounter(&[EnemySpawn {
        character: "cpp".into(),
        x: 650.0,
        facing: Facing::Left,
    }])
    .unwrap();
    let mut moves = std::collections::BTreeSet::new();
    for frame in 450..1050 {
        let input = super::review::input(&sim, frame);
        for event in sim.tick(input) {
            if let Event::Hit { move_id, .. } = event {
                moves.insert(move_id);
            }
        }
    }
    assert!(moves.contains("cpp.light-1"), "{moves:?}");
    assert!(moves.contains("cpp.light-2"), "{moves:?}");
    assert!(moves.contains("cpp.light-3"), "{moves:?}");
}

#[test]
fn reload_keeps_the_scrubbed_pose_after_steps_and_a_changed_clip_duration() {
    let content = ActorContent::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/adventure/actors/cpp/character.json"),
    )
    .unwrap();
    let old = &content.clips.clips["run"];
    // The cursor has moved three frames since the CLI's initial phase.
    let stepped = old.duration_ticks as f32 * 0.125 + 3.0;
    let same = super::reloaded_preview_ticks(stepped, old, old);
    assert_eq!(
        same, stepped,
        "F5 must not restore the invocation's initial phase"
    );
    let phase = stepped / old.duration_ticks as f32;
    let original = content
        .clips
        .sample("run", stepped, phase * old.stride_pixels.unwrap())
        .pose;
    let mut candidate = content.clips.clone();
    candidate.clips.get_mut("run").unwrap().duration_ticks *= 2;
    let new = &candidate.clips["run"];
    let restored = super::reloaded_preview_ticks(stepped, old, new);
    let restored_phase = restored / new.duration_ticks as f32;
    let reloaded = candidate
        .sample("run", restored, restored_phase * new.stride_pixels.unwrap())
        .pose;
    assert_eq!(phase, restored_phase);
    assert_eq!(
        serde_json::to_value(original).unwrap(),
        serde_json::to_value(reloaded).unwrap()
    );
}
