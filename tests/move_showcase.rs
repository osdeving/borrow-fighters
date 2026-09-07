use borrow_fighters::{
    characters::CharacterId,
    scenes::{
        combat_lab::{CombatLabInput, CombatLabMove},
        move_showcase::{MoveShowcase, MoveShowcaseOptions},
    },
};

#[test]
fn showcase_autoplays_to_next_move_after_display_window() {
    let mut showcase = MoveShowcase::default();

    assert_eq!(showcase.selected_move(), CombatLabMove::LightPunch);
    for _ in 0..64 {
        showcase.update(CombatLabInput::default());
    }

    assert_eq!(showcase.selected_move(), CombatLabMove::HeavyPunch);
    assert!(showcase.resting());
    assert_eq!(showcase.lab().current_frame().get(), 0);
}

#[test]
fn showcase_manual_controls_skip_replay_and_pause() {
    let mut showcase = MoveShowcase::new(MoveShowcaseOptions {
        character: CharacterId::Python,
    });

    showcase.update(CombatLabInput {
        next_move: true,
        ..CombatLabInput::default()
    });
    assert_eq!(showcase.character(), CharacterId::Python);
    assert_eq!(showcase.selected_move(), CombatLabMove::HeavyPunch);

    showcase.update(CombatLabInput {
        replay: true,
        ..CombatLabInput::default()
    });
    assert_eq!(showcase.selected_move(), CombatLabMove::HeavyPunch);
    assert_eq!(showcase.lab().current_frame().get(), 1);

    showcase.update(CombatLabInput {
        pause_toggle: true,
        ..CombatLabInput::default()
    });
    let paused_frame = showcase.lab().current_frame();
    showcase.update(CombatLabInput::default());
    assert!(showcase.paused());
    assert_eq!(showcase.lab().current_frame(), paused_frame);
}

#[test]
fn showcase_keeps_baseline_metadata_when_manual_and_automatic_playback_replace_the_lab() {
    let baseline = borrow_fighters::engine::sprites::SpriteManifest::load(
        "assets/placeholder/rust-fighter.sprite.json",
    )
    .unwrap();
    let mut showcase = MoveShowcase::default();
    showcase.set_combat_manifest(Some(baseline.clone()));
    for input in [
        CombatLabInput::default(),
        CombatLabInput {
            next_move: true,
            ..CombatLabInput::default()
        },
        CombatLabInput {
            replay: true,
            ..CombatLabInput::default()
        },
    ] {
        showcase.update(input);
        assert!(showcase.lab().projected_combat().is_some());
    }
    for _ in 0..90 {
        showcase.update(CombatLabInput::default());
    }
    assert_eq!(showcase.selected_move(), CombatLabMove::Kick);
    assert!(showcase.lab().projected_combat().is_some());
}
