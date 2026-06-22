//! Verifies CPU coverage for traditional move inputs without growing combat_rules.rs.

use borrow_fighters::combat::fighter::FighterInput;
use borrow_fighters::config::FIXED_TIMESTEP;
use borrow_fighters::game::{ai::BasicCpu, world::World};

const DT: f32 = FIXED_TIMESTEP;

#[test]
fn cpu_mixes_throw_sweep_and_overhead_inputs_when_close() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 520.0;
    world.player_two.position.x = 580.0;
    let mut cpu = BasicCpu::for_slot(world.player_two.slot);
    let mut seen = TraditionalCpuInputs::default();

    for _ in 0..360 {
        seen.observe(cpu.next_input(&world, world.player_two.slot, DT));
    }

    assert!(seen.throw, "CPU should sometimes try close throw");
    assert!(seen.sweep, "CPU should sometimes try low sweep");
    assert!(
        seen.overhead,
        "CPU should sometimes try forward heavy overhead"
    );
}

#[test]
fn cpu_uses_anti_air_against_airborne_target() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 520.0;
    world.player_one.position.y -= 92.0;
    world.player_one.grounded = false;
    world.player_two.position.x = 580.0;
    let mut cpu = BasicCpu::for_slot(world.player_two.slot);

    let mut anti_air = false;
    for _ in 0..120 {
        let input = cpu.next_input(&world, world.player_two.slot, DT);
        anti_air |= input.crouch && input.heavy_punch;
    }

    assert!(anti_air, "CPU should answer airborne targets with anti-air");
}

#[test]
fn cpu_uses_air_attacks_while_airborne() {
    let mut world = World::new_greybox();
    world.player_one.position.x = 520.0;
    world.player_two.position.x = 580.0;
    world.player_two.position.y -= 92.0;
    world.player_two.grounded = false;
    let mut cpu = BasicCpu::for_slot(world.player_two.slot);

    let mut air_attack = false;
    for _ in 0..120 {
        let input = cpu.next_input(&world, world.player_two.slot, DT);
        air_attack |= input.light_punch || input.kick;
    }

    assert!(air_attack, "CPU should sometimes attack while airborne");
}

#[derive(Default)]
struct TraditionalCpuInputs {
    throw: bool,
    sweep: bool,
    overhead: bool,
}

impl TraditionalCpuInputs {
    fn observe(&mut self, input: FighterInput) {
        self.throw |= input.block && input.light_punch;
        self.sweep |= input.crouch && input.kick;
        self.overhead |= input.heavy_punch && (input.left || input.right);
    }
}
