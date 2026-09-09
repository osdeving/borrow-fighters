//! Runs deterministic mirrored CPU matchups without a window for MVP balance review.
//!
//! System: Playtest tooling. Results expose stalls and gross damage asymmetry;
//! they are not competitive win-rate estimates or a substitute for human play.

use borrow_fighters::characters::CharacterId;
use borrow_fighters::combat::fighter::PlayerSlot;
use borrow_fighters::game::ai::BasicCpu;
use borrow_fighters::game::world::{MatchOutcome, World};

fn main() {
    let roster = [
        CharacterId::Rust,
        CharacterId::Duke,
        CharacterId::C,
        CharacterId::Python,
        CharacterId::Cpp,
    ];
    println!("p1,p2,spacing,winner,seconds,p1_hp,p2_hp");
    for p1 in roster {
        for p2 in roster {
            if p1 == p2 {
                continue;
            }
            for spacing in [0.0, 32.0, 64.0] {
                let mut world = World::new_with_characters(p1, p2);
                world.player_one.position.x -= spacing;
                world.player_two.position.x += spacing;
                let mut first = BasicCpu::for_slot(PlayerSlot::One);
                let mut second = BasicCpu::for_slot(PlayerSlot::Two);
                for _ in 0..60 * 120 {
                    let one = first.next_input(&world, PlayerSlot::One, 1.0 / 60.0);
                    let two = second.next_input(&world, PlayerSlot::Two, 1.0 / 60.0);
                    world.update(1.0 / 60.0, one, two);
                    world.take_audio_events();
                    if world.outcome.is_some() {
                        break;
                    }
                }
                let winner = match world.outcome {
                    Some(MatchOutcome::Winner(PlayerSlot::One)) => p1.audio_key(),
                    Some(MatchOutcome::Winner(PlayerSlot::Two)) => p2.audio_key(),
                    Some(MatchOutcome::Draw) => "draw",
                    None => "timeout",
                };
                println!(
                    "{},{},{spacing},{winner},{:.2},{},{}",
                    p1.audio_key(),
                    p2.audio_key(),
                    world.elapsed_seconds,
                    world.player_one.health,
                    world.player_two.health
                );
            }
        }
    }
}
