//! Verifies table-driven close-range move data.

use borrow_fighters::combat::fighter::{
    AttackKind, HEAVY_PUNCH_DAMAGE, KICK_DAMAGE, LIGHT_PUNCH_DAMAGE,
};
use borrow_fighters::combat::frame::FrameCount;
use borrow_fighters::combat::move_data::{
    CLOSE_RANGE_MOVE_SPECS, MoveId, MoveInputKind, move_spec,
};

#[test]
fn close_range_moves_are_registered_in_table_order() {
    assert_eq!(CLOSE_RANGE_MOVE_SPECS.len(), 3);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[0].id, MoveId::LightPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[1].id, MoveId::HeavyPunch);
    assert_eq!(CLOSE_RANGE_MOVE_SPECS[2].id, MoveId::Kick);
}

#[test]
fn move_specs_preserve_current_tuning_values() {
    let light = move_spec(MoveId::LightPunch);
    assert_eq!(light.input, MoveInputKind::LightPunch);
    assert_eq!(light.label, "LP");
    assert_eq!(light.damage, LIGHT_PUNCH_DAMAGE);
    assert_eq!(light.frames.duration, FrameCount::new(18));
    assert_eq!(light.frames.active_start, FrameCount::new(5));
    assert_eq!(light.frames.active_end, FrameCount::new(10));
    assert_eq!(light.hitbox.width, 58.0);
    assert_eq!(light.hitbox.height, 34.0);
    assert_eq!(light.hitbox.y_offset, 62.0);

    let heavy = move_spec(MoveId::HeavyPunch);
    assert_eq!(heavy.input, MoveInputKind::HeavyPunch);
    assert_eq!(heavy.label, "HP");
    assert_eq!(heavy.damage, HEAVY_PUNCH_DAMAGE);
    assert_eq!(heavy.frames.duration, FrameCount::new(35));
    assert_eq!(heavy.frames.active_start, FrameCount::new(11));
    assert_eq!(heavy.frames.active_end, FrameCount::new(20));
    assert_eq!(heavy.hitbox.width, 96.0);
    assert_eq!(heavy.hitbox.height, 42.0);
    assert_eq!(heavy.hitbox.y_offset, 58.0);

    let kick = move_spec(MoveId::Kick);
    assert_eq!(kick.input, MoveInputKind::Kick);
    assert_eq!(kick.label, "KICK");
    assert_eq!(kick.damage, KICK_DAMAGE);
    assert_eq!(kick.frames.duration, FrameCount::new(28));
    assert_eq!(kick.frames.active_start, FrameCount::new(9));
    assert_eq!(kick.frames.active_end, FrameCount::new(16));
    assert_eq!(kick.hitbox.width, 100.0);
    assert_eq!(kick.hitbox.height, 36.0);
    assert_eq!(kick.hitbox.y_offset, 108.0);
}

#[test]
fn attack_kind_is_compatibility_layer_over_move_specs() {
    assert_eq!(AttackKind::LightPunch.move_id(), MoveId::LightPunch);
    assert_eq!(
        AttackKind::LightPunch.move_spec(),
        move_spec(MoveId::LightPunch)
    );
    assert_eq!(
        AttackKind::HeavyPunch.move_spec(),
        move_spec(MoveId::HeavyPunch)
    );
    assert_eq!(AttackKind::Kick.move_spec(), move_spec(MoveId::Kick));
}
