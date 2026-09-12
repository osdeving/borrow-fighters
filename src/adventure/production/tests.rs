//! Exercises production combat through the same public API used by lab and chapter.
//!
//! Fixtures load the shipped JSON; narrow modifications isolate timing and contact
//! boundaries without depending on renderer internals or a second combat model.

use super::*;
use crate::math::vec2::Vec2;
use std::sync::Arc;

fn packs() -> Vec<CharacterPack> {
    [
        (
            include_str!("../../../assets/adventure/actors/cpp/character.json"),
            include_str!("../../../assets/adventure/actors/cpp/combat.json"),
        ),
        (
            include_str!("../../../assets/adventure/actors/security/character.json"),
            include_str!("../../../assets/adventure/actors/security/combat.json"),
        ),
        (
            include_str!("../../../assets/adventure/actors/erratic/character.json"),
            include_str!("../../../assets/adventure/actors/erratic/combat.json"),
        ),
    ]
    .into_iter()
    .map(|(character, combat)| CharacterPack::from_json(character, combat).unwrap())
    .collect()
}

fn content() -> Arc<CombatCatalog> {
    Arc::new(CombatCatalog::new(packs()).unwrap())
}

fn controlled_content() -> Arc<CombatCatalog> {
    let mut packs = packs();
    for p in &mut packs {
        for m in &mut p.combat.moves {
            m.hitstop = 0;
            m.knockback = 0.0;
        }
    }
    Arc::new(CombatCatalog::new(packs).unwrap())
}

fn simulation(content: Arc<CombatCatalog>) -> Simulation {
    Simulation::new(
        content,
        Bounds {
            left: 0.0,
            right: 3000.0,
            floor_y: 600.0,
        },
        "cpp",
        200.0,
    )
    .unwrap()
}

fn enemy(sim: &mut Simulation, character: &str, x: f32) -> ActorId {
    sim.spawn_enemy(
        &EnemySpawn {
            character: character.into(),
            x,
            facing: Facing::Left,
        },
        true,
    )
    .unwrap()
}

/// Explicitly idles enemies so a contact test cannot be changed by autonomous AI.
fn step(sim: &mut Simulation, player: Input, overrides: &[(ActorId, Input)]) -> Vec<Event> {
    let mut controls: Vec<_> = sim
        .actors()
        .iter()
        .map(|a| (a.id, Input::default()))
        .collect();
    controls[0].1 = player;
    for (id, input) in overrides {
        controls
            .iter_mut()
            .find(|(actor, _)| actor == id)
            .unwrap()
            .1 = *input;
    }
    sim.tick_with_controls(&controls)
}

fn ticks(sim: &mut Simulation, n: u32) -> Vec<Event> {
    (0..n)
        .flat_map(|_| step(sim, Input::default(), &[]))
        .collect()
}

#[test]
fn shipped_packs_define_three_distinct_roles_and_a_complete_light_chain() {
    let c = content();
    assert_eq!(c.characters.len(), 3);
    assert_eq!(c.moves.len(), 8);
    assert_eq!(c.characters["cpp"].loadout.light.len(), 3);
    assert!(c.characters["erratic"].hp > c.characters["security"].hp);
    assert!(c.moves["erratic.light-1"].startup < c.moves["security.light-1"].startup);
    assert_eq!(
        c.moves["cpp.light-1"].combo_next.as_deref(),
        Some("cpp.light-2")
    );
}

#[test]
fn malformed_versions_timing_paths_and_numbers_are_rejected_before_use() {
    let mutations: [fn(&mut CharacterPack); 7] = [
        |p| p.character.schema_version = 2,
        |p| p.combat.tick_hz = 30,
        |p| p.character.rig = "../foreign/rig.json".into(),
        |p| p.character.movement.acceleration = f32::NAN,
        |p| p.combat.moves[0].hitboxes[0].start = 0,
        |p| p.combat.moves[0].hitboxes[0].rect[2] = 0.0,
        |p| p.character.guard.parry_cooldown = 0,
    ];
    for mutation in mutations {
        let mut candidate = packs();
        mutation(&mut candidate[0]);
        assert!(CombatCatalog::new(candidate).is_err());
    }
    let source = include_str!("../../../assets/adventure/actors/cpp/character.json");
    let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
    value["unrecognized_contract_field"] = true.into();
    assert!(
        CharacterPack::from_json(
            &value.to_string(),
            include_str!("../../../assets/adventure/actors/cpp/combat.json")
        )
        .is_err()
    );
}

#[test]
fn references_duplicate_ids_and_cyclic_combos_are_rejected() {
    let mut candidate = packs();
    candidate[0].character.loadout.kick = Some("missing".into());
    assert!(CombatCatalog::new(candidate).is_err());
    let mut candidate = packs();
    candidate.push(candidate[0].clone());
    assert!(CombatCatalog::new(candidate).is_err());
    let mut candidate = packs();
    candidate[0].combat.moves[2].combo_next = Some("cpp.light-1".into());
    candidate[0].combat.moves[2].combo_window = Some([14, 34]);
    assert!(CombatCatalog::new(candidate).is_err());
    let mut candidate = packs();
    candidate[0].combat.moves[5]
        .projectile
        .as_mut()
        .unwrap()
        .spawn_tick = 15;
    assert!(CombatCatalog::new(candidate).is_err());
}

#[test]
fn start_stop_and_turn_follow_actual_acceleration_and_ground_distance() {
    let mut sim = simulation(content());
    step(
        &mut sim,
        Input {
            movement: 1.0,
            ..Input::default()
        },
        &[],
    );
    assert_eq!(sim.player().action, Action::Start);
    assert!(sim.player().velocity.x > 0.0 && sim.player().velocity.x < 360.0);
    for _ in 0..15 {
        step(
            &mut sim,
            Input {
                movement: 1.0,
                ..Input::default()
            },
            &[],
        );
    }
    assert_eq!(sim.player().action, Action::Run);
    assert!((sim.player().stride_distance - (sim.player().position.x - 200.0)).abs() < 0.001);
    step(
        &mut sim,
        Input {
            movement: -1.0,
            ..Input::default()
        },
        &[],
    );
    assert_eq!(
        sim.player().facing,
        Facing::Right,
        "body must finish its deceleration before turning"
    );
    for _ in 0..8 {
        step(
            &mut sim,
            Input {
                movement: -1.0,
                ..Input::default()
            },
            &[],
        );
    }
    assert_eq!(sim.player().facing, Facing::Left);
    assert_eq!(sim.player().action, Action::Turn);
    ticks(&mut sim, 20);
    assert_eq!(sim.player().velocity, Vec2::ZERO);
    assert_eq!(sim.player().action, Action::Idle);
}

#[test]
fn walking_into_a_wall_does_not_advance_stride_and_nonfinite_axis_is_neutral() {
    let mut sim = simulation(content());
    sim.stage_player(4000.0, Facing::Right).unwrap();
    let at = sim.player().position;
    for _ in 0..60 {
        step(
            &mut sim,
            Input {
                movement: 1.0,
                ..Input::default()
            },
            &[],
        );
    }
    assert_eq!(sim.player().position, at);
    assert_eq!(sim.player().stride_distance, 0.0);
    assert_eq!(sim.player().velocity, Vec2::ZERO);
    step(
        &mut sim,
        Input {
            movement: f32::NAN,
            ..Input::default()
        },
        &[],
    );
    assert_eq!(sim.player().position, at);
}

#[test]
fn jump_has_one_landing_event_and_returns_to_the_same_floor() {
    let mut sim = simulation(content());
    step(
        &mut sim,
        Input {
            jump: true,
            ..Input::default()
        },
        &[],
    );
    assert!(!sim.player().grounded);
    assert_eq!(sim.player().action, Action::Jump);
    let events = ticks(&mut sim, 100);
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e, Event::Landed { .. }))
            .count(),
        1
    );
    assert_eq!(sim.player().position.y, 600.0);
    assert!(sim.player().grounded);
    assert_eq!(sim.player().action, Action::Idle);
    assert_eq!(sim.player().stride_distance, 0.0);
}

#[test]
fn attack_windows_are_half_open_and_one_swing_hits_a_target_once() {
    let mut sim = simulation(controlled_content());
    let id = enemy(&mut sim, "security", 260.0);
    step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    assert!(sim.hitboxes(sim.player_id()).is_empty());
    ticks(&mut sim, 5);
    assert!(sim.hitboxes(sim.player_id()).is_empty());
    let mut events = step(&mut sim, Input::default(), &[]);
    assert_eq!(sim.player().action_ticks, 6);
    assert!(!sim.hitboxes(sim.player_id()).is_empty());
    events.extend(ticks(&mut sim, 30));
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e,Event::Hit {target,..} if *target==id))
            .count(),
        1
    );
    assert_eq!(sim.actor(id).unwrap().hp, 65 - 12);
    assert!(sim.hitboxes(sim.player_id()).is_empty());
}

#[test]
fn a_buffered_light_chain_executes_three_distinct_moves_and_damage_values() {
    let mut sim = simulation(controlled_content());
    let id = enemy(&mut sim, "erratic", 260.0);
    let mut events = step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    let mut queued_second = false;
    let mut queued_third = false;
    for _ in 0..100 {
        let player = sim.player();
        let light = if player.move_id.as_deref() == Some("cpp.light-1")
            && player.action_ticks == 8
            && !queued_second
        {
            queued_second = true;
            true
        } else if player.move_id.as_deref() == Some("cpp.light-2")
            && player.action_ticks == 9
            && !queued_third
        {
            queued_third = true;
            true
        } else {
            false
        };
        events.extend(step(
            &mut sim,
            Input {
                light,
                ..Input::default()
            },
            &[],
        ));
    }
    let moves: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            Event::MoveStarted { actor, move_id } if *actor == sim.player_id() => {
                Some(move_id.as_str())
            }
            _ => None,
        })
        .collect();
    assert_eq!(moves, ["cpp.light-1", "cpp.light-2", "cpp.light-3"]);
    assert_eq!(sim.actor(id).unwrap().hp, 105 - 12 - 14 - 22);
}

#[test]
fn expired_early_input_does_not_automatically_queue_another_move() {
    let mut sim = simulation(controlled_content());
    step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    let mut events = step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    events.extend(ticks(&mut sim, 80));
    assert!(
        !events
            .iter()
            .any(|e| matches!(e, Event::MoveStarted { .. }))
    );
}

#[test]
fn kick_mirrors_its_forward_box_and_spin_hits_both_sides_once() {
    let mut sim = simulation(controlled_content());
    sim.stage_player(200.0, Facing::Left).unwrap();
    let left = enemy(&mut sim, "security", 120.0);
    let right = enemy(&mut sim, "security", 280.0);
    step(
        &mut sim,
        Input {
            kick: true,
            ..Input::default()
        },
        &[],
    );
    ticks(&mut sim, 40);
    assert_eq!(sim.actor(left).unwrap().hp, 43);
    assert_eq!(sim.actor(right).unwrap().hp, 65);
    step(
        &mut sim,
        Input {
            spin: true,
            ..Input::default()
        },
        &[],
    );
    let events = ticks(&mut sim, 50);
    assert_eq!(sim.actor(left).unwrap().hp, 19);
    assert_eq!(sim.actor(right).unwrap().hp, 41);
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e, Event::Hit { .. }))
            .count(),
        2
    );
}

#[test]
fn sustained_guard_blocks_front_but_a_rear_attack_deals_damage() {
    for facing in [Facing::Right, Facing::Left] {
        let mut sim = simulation(controlled_content());
        let id = enemy(&mut sim, "security", 260.0);
        sim.stage_player(200.0, facing).unwrap();
        let guard = Input {
            guard: true,
            ..Input::default()
        };
        for _ in 0..30 {
            step(&mut sim, guard, &[]);
        }
        step(
            &mut sim,
            guard,
            &[(
                id,
                Input {
                    light: true,
                    ..Input::default()
                },
            )],
        );
        let events: Vec<_> = (0..30).flat_map(|_| step(&mut sim, guard, &[])).collect();
        if facing == Facing::Right {
            assert_eq!(sim.player().hp, 160);
            assert_eq!(
                events
                    .iter()
                    .filter(|e| matches!(e, Event::Blocked { .. }))
                    .count(),
                1
            );
        } else {
            assert_eq!(sim.player().hp, 149);
            assert!(
                events
                    .iter()
                    .any(|e| matches!(e,Event::Hit {target,..} if *target==sim.player_id()))
            );
        }
    }
}

#[test]
fn fresh_guard_parries_a_telegraph_and_holding_does_not_rearm_parry() {
    let mut sim = simulation(controlled_content());
    let id = enemy(&mut sim, "security", 260.0);
    step(
        &mut sim,
        Input::default(),
        &[(
            id,
            Input {
                light: true,
                ..Input::default()
            },
        )],
    );
    ticks(&mut sim, 20);
    let guard = Input {
        guard: true,
        ..Input::default()
    };
    let events: Vec<_> = (0..4).flat_map(|_| step(&mut sim, guard, &[])).collect();
    assert_eq!(sim.player().hp, 160);
    assert!(
        events
            .iter()
            .any(|e| matches!(e,Event::Parried {defender,..} if *defender==sim.player_id()))
    );
    assert_eq!(sim.actor(id).unwrap().action, Action::Hurt);
    for _ in 0..50 {
        step(&mut sim, guard, &[]);
    }
    step(
        &mut sim,
        guard,
        &[(
            id,
            Input {
                light: true,
                ..Input::default()
            },
        )],
    );
    let events: Vec<_> = (0..30).flat_map(|_| step(&mut sim, guard, &[])).collect();
    assert!(events.iter().any(|e| matches!(e, Event::Blocked { .. })));
    assert!(!events.iter().any(|e| matches!(e, Event::Parried { .. })));
}

#[test]
fn linker_spawns_once_and_sweeps_the_nearest_target_between_frames() {
    let mut packs = packs();
    let projectile = packs[0].combat.moves[5].projectile.as_mut().unwrap();
    projectile.velocity[0] = 4000.0;
    projectile.rect = [-1.0, -1.0, 2.0, 2.0];
    packs[1].character.body_width = 1.0;
    packs[1].character.hurtboxes = vec![[-1.0, -125.0, 2.0, 25.0]];
    let mut sim = simulation(Arc::new(CombatCatalog::new(packs).unwrap()));
    let farther = enemy(&mut sim, "security", 305.0);
    let nearer = enemy(&mut sim, "security", 290.0);
    step(
        &mut sim,
        Input {
            linker: true,
            ..Input::default()
        },
        &[],
    );
    let events = ticks(&mut sim, 80);
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e, Event::ProjectileSpawned { .. }))
            .count(),
        1
    );
    assert_eq!(sim.actor(nearer).unwrap().hp, 47);
    assert_eq!(sim.actor(farther).unwrap().hp, 65);
    assert!(sim.projectiles().is_empty());
}

#[test]
fn linker_ignores_allies_and_guard_intercepts_a_projectile() {
    let mut sim = simulation(controlled_content());
    let caster = enemy(&mut sim, "cpp", 500.0);
    let ally = enemy(&mut sim, "security", 360.0);
    let guard = Input {
        guard: true,
        ..Input::default()
    };
    for _ in 0..30 {
        step(&mut sim, guard, &[]);
    }
    step(
        &mut sim,
        guard,
        &[(
            caster,
            Input {
                linker: true,
                ..Input::default()
            },
        )],
    );
    let events: Vec<_> = (0..80).flat_map(|_| step(&mut sim, guard, &[])).collect();
    assert_eq!(sim.actor(ally).unwrap().hp, 65);
    assert_eq!(sim.player().hp, 160);
    assert!(
        events
            .iter()
            .any(|e| matches!(e,Event::Blocked {attacker,..} if *attacker==caster))
    );
    assert!(sim.projectiles().is_empty());
}

#[test]
fn hitstop_preserves_pose_and_buffers_the_next_attack_input() {
    let mut sim = simulation(content());
    enemy(&mut sim, "erratic", 260.0);
    step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    ticks(&mut sim, 6);
    assert_eq!(sim.hitstop_remaining, 3);
    let pose = (
        sim.player().position,
        sim.player().action_ticks,
        sim.player().stride_distance,
    );
    step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    assert_eq!(
        (
            sim.player().position,
            sim.player().action_ticks,
            sim.player().stride_distance
        ),
        pose
    );
    let events = ticks(&mut sim, 14);
    assert!(
        events
            .iter()
            .any(|e| matches!(e,Event::MoveStarted {move_id,..} if move_id=="cpp.light-2"))
    );
}

#[test]
fn staged_enemies_count_as_alive_but_cannot_take_or_deal_damage() {
    let mut sim = simulation(controlled_content());
    let id = sim
        .spawn_enemy(
            &EnemySpawn {
                character: "erratic".into(),
                x: 260.0,
                facing: Facing::Left,
            },
            false,
        )
        .unwrap();
    sim.set_actor_position(id, Vec2::new(260.0, 200.0), Facing::Left)
        .unwrap();
    assert_eq!(sim.actor(id).unwrap().clip_id(), "arrival");
    assert_eq!(sim.outcome(), Outcome::Ongoing);
    step(
        &mut sim,
        Input {
            spin: true,
            ..Input::default()
        },
        &[],
    );
    ticks(&mut sim, 60);
    assert_eq!(sim.actor(id).unwrap().hp, 105);
    assert_eq!(sim.actor(id).unwrap().position.y, 200.0);
    assert!(sim.hurtboxes(id).is_empty());
    sim.set_actor_position(id, Vec2::new(260.0, 600.0), Facing::Left)
        .unwrap();
    sim.set_actor_active(id, true).unwrap();
    assert_eq!(sim.actor(id).unwrap().clip_id(), "idle");
    assert!(!sim.hurtboxes(id).is_empty());
}

#[test]
fn wave_replacement_is_transactional_and_preserves_player_health() {
    let mut sim = simulation(controlled_content());
    let previous = enemy(&mut sim, "security", 260.0);
    step(
        &mut sim,
        Input::default(),
        &[(
            previous,
            Input {
                light: true,
                ..Input::default()
            },
        )],
    );
    ticks(&mut sim, 30);
    assert_eq!(sim.player().hp, 149);
    let candidate = EnemySpawn {
        character: "erratic".into(),
        x: 1000.0,
        facing: Facing::Left,
    };
    assert!(
        sim.begin_encounter(&[
            candidate.clone(),
            EnemySpawn {
                character: "missing".into(),
                ..candidate.clone()
            }
        ])
        .is_err()
    );
    assert!(sim.actor(previous).is_some());
    let next = sim.begin_encounter(&[candidate]).unwrap();
    assert!(sim.actor(previous).is_none());
    assert!(next[0].0 > previous.0);
    assert_eq!(sim.player().hp, 149);
    assert_eq!(sim.player().action, Action::Idle);
    assert_eq!(sim.outcome(), Outcome::Ongoing);
    sim.clear_encounter();
    assert_eq!(sim.actors().len(), 1);
    assert_eq!(sim.outcome(), Outcome::Ongoing);
}

#[test]
fn knockout_is_reported_once_and_checkpoint_reset_removes_transients() {
    let mut packs = packs();
    packs[1].character.hp = 10;
    let mut sim = simulation(Arc::new(CombatCatalog::new(packs).unwrap()));
    let id = enemy(&mut sim, "security", 260.0);
    step(
        &mut sim,
        Input {
            light: true,
            ..Input::default()
        },
        &[],
    );
    let events = ticks(&mut sim, 80);
    assert_eq!(sim.outcome(), Outcome::Victory);
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e,Event::Knockout {actor,..} if *actor==id))
            .count(),
        1
    );
    assert!(sim.hurtboxes(id).is_empty());
    step(
        &mut sim,
        Input {
            linker: true,
            ..Input::default()
        },
        &[],
    );
    ticks(&mut sim, 17);
    assert!(!sim.projectiles().is_empty());
    sim.reset_player(300.0).unwrap();
    assert_eq!(sim.actors().len(), 1);
    assert!(sim.projectiles().is_empty());
    assert_eq!(sim.player().hp, 160);
    assert_eq!(sim.player().position, Vec2::new(300.0, 600.0));
    assert_eq!(sim.player().action, Action::Idle);
    assert_eq!(sim.player().stride_distance, 0.0);
    assert_eq!(sim.hitstop_remaining, 0);
    assert_eq!(sim.outcome(), Outcome::Ongoing);
}

#[test]
fn identical_script_and_ai_produce_identical_snapshots_and_events() {
    let content = content();
    let mut first = simulation(content.clone());
    let mut second = simulation(content);
    let spawns = [
        EnemySpawn {
            character: "security".into(),
            x: 500.0,
            facing: Facing::Left,
        },
        EnemySpawn {
            character: "erratic".into(),
            x: 800.0,
            facing: Facing::Left,
        },
    ];
    first.begin_encounter(&spawns).unwrap();
    second.begin_encounter(&spawns).unwrap();
    for tick in 0..900 {
        let input = Input {
            movement: if tick % 240 < 120 { 1.0 } else { -1.0 },
            light: tick % 31 == 0,
            kick: tick % 113 == 0,
            spin: tick % 149 == 0,
            linker: tick % 211 == 0,
            guard: tick % 71 > 60,
            jump: tick % 173 == 0,
        };
        assert_eq!(first.tick(input), second.tick(input));
        assert_eq!(
            format!("{:?}", first.actors()),
            format!("{:?}", second.actors())
        );
        assert_eq!(
            format!("{:?}", first.projectiles()),
            format!("{:?}", second.projectiles())
        );
    }
}
