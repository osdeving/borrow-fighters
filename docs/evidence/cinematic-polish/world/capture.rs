//! Native inspection of facade joints at several camera positions.
use borrow_fighters::adventure::{
    engine::{assets::Assets, render},
    story::{Stage, Story},
    text::TextCatalog,
};
use raylib::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Street join review")
        .hidden()
        .build();
    let assets = Assets::load(
        &mut rl,
        &thread,
        TextCatalog::load("assets/adventure/texts/pt-BR.json")?,
    )?;
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    let mut story = Story::new();
    story.stage = Stage::Encounter;
    story.stage_ticks = 10000;
    for (name, x) in [
        ("start", 160.0),
        ("middle", 1730.0),
        ("neighborhood", 3270.0),
        ("end", 4410.0),
    ] {
        story.combat.player.position.x = x;
        {
            let mut d = rl.begin_texture_mode(&thread, &mut target);
            render::draw(&mut d, &story, &assets, false, false, false);
        }
        let mut frame = target.texture().load_image()?;
        frame.flip_vertical();
        frame.export_image(&format!("docs/evidence/cinematic-polish/world/{name}.png"));
    }
    Ok(())
}
