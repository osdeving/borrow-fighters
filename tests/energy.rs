//! Verifies cinematic payment and energy earned only from real combat contacts.
//!
//! System: Match integration. Tests exercise World resolution without rendering,
//! including tools, both player slots, disabled HP damage, and local Go supers.

use borrow_fighters::{
    characters::CharacterId,
    combat::{
        fighter::{FighterInput, PlayerSlot},
        projectile::Projectile,
    },
    config::{FIXED_TIMESTEP as DT, world_px},
    game::{
        energy::{
            BLOCK_ENERGY, BLOCKED_ATTACK_ENERGY, EnergyPolicy, HIT_DEALT_ENERGY,
            HIT_RECEIVED_ENERGY, INITIAL_ENERGY, MAX_ENERGY,
        },
        feature_flags::{FeatureFlag, FeatureFlags},
        world::{MatchOutcome, World},
    },
    math::vec2::Vec2,
};

const ROSTER: [CharacterId; 6] = [
    CharacterId::Rust,
    CharacterId::Duke,
    CharacterId::Go,
    CharacterId::C,
    CharacterId::Python,
    CharacterId::Cpp,
];
const SLOTS: [PlayerSlot; 2] = [PlayerSlot::One, PlayerSlot::Two];

fn other(slot: PlayerSlot) -> PlayerSlot {
    match slot {
        PlayerSlot::One => PlayerSlot::Two,
        PlayerSlot::Two => PlayerSlot::One,
    }
}

fn no_damage() -> FeatureFlags {
    let mut flags = FeatureFlags::default();
    flags.set(FeatureFlag::PlayerOneTakesDamage, false);
    flags.set(FeatureFlag::PlayerTwoTakesDamage, false);
    flags
}

fn step(world: &mut World, one: FighterInput, two: FighterInput) {
    world.update_with_flags(DT, one, two, no_damage());
}

fn rest(world: &mut World, frames: usize) {
    for _ in 0..frames {
        step(world, FighterInput::default(), FighterInput::default());
    }
}

fn cinematic(world: &mut World, slot: PlayerSlot) {
    let request = FighterInput {
        cinematic_special: true,
        ..Default::default()
    };
    match slot {
        PlayerSlot::One => step(world, request, FighterInput::default()),
        PlayerSlot::Two => step(world, FighterInput::default(), request),
    }
}

fn close_pair(world: &mut World) {
    world.player_one.position.x = world_px(300.0);
    world.player_two.position.x = world.player_one.body_rect().right() + world_px(10.0);
}

fn inject_contact(world: &mut World, attacker: PlayerSlot, blocked: bool) {
    let (actor, target) = match attacker {
        PlayerSlot::One => (&world.player_one, &world.player_two),
        PlayerSlot::Two => (&world.player_two, &world.player_one),
    };
    let mut projectile = Projectile::from_fighter(actor);
    projectile.position = Vec2::new(
        target.body_rect().center_x() - projectile.width * 0.5,
        target.body_rect().y + world_px(65.0),
    );
    projectile.velocity = Vec2::ZERO;
    world.projectiles.push(projectile);
    let guard = FighterInput {
        block: blocked,
        ..Default::default()
    };
    match attacker {
        PlayerSlot::One => step(world, FighterInput::default(), guard),
        PlayerSlot::Two => step(world, guard, FighterInput::default()),
    }
}

fn charge(world: &mut World, slot: PlayerSlot) {
    for _ in 0..5 {
        inject_contact(world, slot, false);
        rest(world, 40);
    }
    assert_eq!(world.energy(slot).amount(), MAX_ENERGY);
}

#[test]
fn every_character_and_slot_requires_full_energy_and_pays_once_on_accepted_entry() {
    for character in ROSTER {
        for slot in SLOTS {
            let mut world = match slot {
                PlayerSlot::One => World::new_with_characters(character, CharacterId::Rust),
                PlayerSlot::Two => World::new_with_characters(CharacterId::Rust, character),
            };
            world.set_energy_policy(EnergyPolicy::Metered);
            assert_eq!(world.energy(slot).fraction(), 0.5);
            assert!(!world.cinematic_ready(slot));
            cinematic(&mut world, slot);
            assert!(!world.super_sequence_active());
            assert!(world.player_one.cinematic_special().is_none());
            assert!(world.player_two.cinematic_special().is_none());
            assert_eq!(world.energy(slot).amount(), INITIAL_ENERGY);
            charge(&mut world, slot);
            close_pair(&mut world);
            let defender_before = world.energy(other(slot));
            cinematic(&mut world, slot);
            assert_eq!(world.energy(slot).amount(), 0, "{character:?}/{slot:?}");
            assert!(
                world.super_sequence_active()
                    || world.player_one.cinematic_special().is_some()
                    || world.player_two.cinematic_special().is_some()
            );
            for _ in 0..700 {
                cinematic(&mut world, slot);
            }
            assert_eq!(world.energy(slot).amount(), 0);
            assert_eq!(world.energy(other(slot)), defender_before);
            assert_eq!(world.player_one.health, world.player_one.max_health);
            assert_eq!(world.player_two.health, world.player_two.max_health);
        }
    }
}

#[test]
fn real_projectile_hits_and_blocks_charge_both_slots_even_with_hp_damage_disabled() {
    for attacker in SLOTS {
        for blocked in [false, true] {
            let mut world = World::new_greybox();
            world.set_energy_policy(EnergyPolicy::Metered);
            inject_contact(&mut world, attacker, blocked);
            assert_eq!(
                world.energy(attacker).amount(),
                INITIAL_ENERGY
                    + if blocked {
                        BLOCKED_ATTACK_ENERGY
                    } else {
                        HIT_DEALT_ENERGY
                    }
            );
            assert_eq!(
                world.energy(other(attacker)).amount(),
                INITIAL_ENERGY
                    + if blocked {
                        BLOCK_ENERGY
                    } else {
                        HIT_RECEIVED_ENERGY
                    }
            );
            let before = [world.energy(PlayerSlot::One), world.energy(PlayerSlot::Two)];
            rest(&mut world, 180);
            assert_eq!(
                before,
                [world.energy(PlayerSlot::One), world.energy(PlayerSlot::Two)],
                "consumed projectile cannot farm energy"
            );
            assert_eq!(world.player_one.health, world.player_one.max_health);
            assert_eq!(world.player_two.health, world.player_two.max_health);
        }
    }
}

#[test]
fn ordinary_melee_and_signature_contacts_charge_once_without_using_the_diagnostic_log() {
    for signature in [false, true] {
        for blocked in [false, true] {
            let mut world = World::new_with_characters(CharacterId::C, CharacterId::Rust);
            world.set_energy_policy(EnergyPolicy::Metered);
            close_pair(&mut world);
            let attack = FighterInput {
                light_punch: !signature,
                signature_special: signature,
                ..Default::default()
            };
            for tick in 0..180 {
                step(
                    &mut world,
                    if tick == 0 {
                        attack
                    } else {
                        FighterInput::default()
                    },
                    FighterInput {
                        block: blocked,
                        crouch: signature && blocked,
                        ..Default::default()
                    },
                );
                world.clear_combat_log();
            }
            assert_eq!(
                world.energy(PlayerSlot::One).amount(),
                INITIAL_ENERGY
                    + if blocked {
                        BLOCKED_ATTACK_ENERGY
                    } else {
                        HIT_DEALT_ENERGY
                    },
                "signature={signature}/blocked={blocked}"
            );
            assert_eq!(
                world.energy(PlayerSlot::Two).amount(),
                INITIAL_ENERGY
                    + if blocked {
                        BLOCK_ENERGY
                    } else {
                        HIT_RECEIVED_ENERGY
                    }
            );
        }
    }
}

#[test]
fn idle_whiffs_and_completed_rounds_never_generate_energy() {
    let mut world = World::new_greybox();
    world.set_energy_policy(EnergyPolicy::Metered);
    for _ in 0..600 {
        step(
            &mut world,
            FighterInput {
                light_punch: true,
                ..Default::default()
            },
            FighterInput::default(),
        );
    }
    for slot in SLOTS {
        assert_eq!(world.energy(slot).amount(), INITIAL_ENERGY);
    }
    world.outcome = Some(MatchOutcome::Draw);
    inject_contact(&mut world, PlayerSlot::One, false);
    for slot in SLOTS {
        assert_eq!(world.energy(slot).amount(), INITIAL_ENERGY);
    }
}

#[test]
fn simultaneous_metered_requests_cancel_without_spending_for_all_six_characters() {
    for one in ROSTER {
        for two in ROSTER {
            let mut world = World::new_with_characters(one, two);
            world.set_energy_policy(EnergyPolicy::Metered);
            charge(&mut world, PlayerSlot::One);
            charge(&mut world, PlayerSlot::Two);
            let request = FighterInput {
                cinematic_special: true,
                ..Default::default()
            };
            step(&mut world, request, request);
            assert!(!world.super_sequence_active());
            assert!(world.player_one.cinematic_special().is_none());
            assert!(world.player_two.cinematic_special().is_none());
            for slot in SLOTS {
                assert_eq!(world.energy(slot).amount(), MAX_ENERGY);
            }
            cinematic(&mut world, PlayerSlot::Two);
            assert_eq!(world.energy(PlayerSlot::Two).amount(), 0);
            assert_eq!(world.energy(PlayerSlot::One).amount(), MAX_ENERGY);
        }
    }
}

#[test]
fn busy_and_airborne_requests_do_not_spend_and_policy_selection_does_not_refill() {
    for character in ROSTER {
        for airborne in [false, true] {
            let mut world = World::new_with_characters(character, CharacterId::Rust);
            world.set_energy_policy(EnergyPolicy::Metered);
            charge(&mut world, PlayerSlot::One);
            if airborne {
                world.player_one.grounded = false;
                world.player_one.position.y -= world_px(80.0);
            } else {
                step(
                    &mut world,
                    FighterInput {
                        light_punch: true,
                        ..Default::default()
                    },
                    FighterInput::default(),
                );
            }
            cinematic(&mut world, PlayerSlot::One);
            assert_eq!(world.energy(PlayerSlot::One).amount(), MAX_ENERGY);
            assert!(!world.super_sequence_active());
            assert!(world.player_one.cinematic_special().is_none());
            world.set_energy_policy(EnergyPolicy::Metered);
            assert_eq!(world.energy(PlayerSlot::One).amount(), MAX_ENERGY);
            world.reset_energy();
            assert_eq!(world.energy_policy(), EnergyPolicy::Metered);
            for slot in SLOTS {
                assert_eq!(world.energy(slot).amount(), INITIAL_ENERGY);
            }
        }
    }
}

#[test]
fn go_cinematic_interruption_and_existing_projectiles_cannot_recharge_either_fighter() {
    for attacker in SLOTS {
        let mut world = World::new_with_characters(CharacterId::Go, CharacterId::Rust);
        world.set_energy_policy(EnergyPolicy::Metered);
        charge(&mut world, PlayerSlot::One);
        cinematic(&mut world, PlayerSlot::One);
        assert_eq!(world.energy(PlayerSlot::One).amount(), 0);
        let before = [world.energy(PlayerSlot::One), world.energy(PlayerSlot::Two)];
        inject_contact(&mut world, attacker, false);
        assert_eq!(
            before,
            [world.energy(PlayerSlot::One), world.energy(PlayerSlot::Two)]
        );
    }
}

#[test]
fn unrestricted_worlds_keep_tools_free_and_new_matches_discard_charge_and_capture() {
    for character in ROSTER {
        let mut world = World::new_with_characters(character, CharacterId::Rust);
        assert_eq!(world.energy_policy(), EnergyPolicy::Unlimited);
        assert!(world.cinematic_ready(PlayerSlot::One));
        cinematic(&mut world, PlayerSlot::One);
        assert_eq!(world.energy(PlayerSlot::One).amount(), INITIAL_ENERGY);
        assert!(world.super_sequence_active() || world.player_one.cinematic_special().is_some());
        world = World::new_with_characters(character, CharacterId::Rust);
        world.set_energy_policy(EnergyPolicy::Metered);
        assert!(!world.super_sequence_active());
        for slot in SLOTS {
            assert_eq!(world.energy(slot).amount(), INITIAL_ENERGY);
        }
    }
}
