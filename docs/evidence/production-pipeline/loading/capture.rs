//! Native asset-scope regression: real loading and unchanged actor/world drawing.
use borrow_fighters::adventure::{
    chapter::{Chapter, Checkpoint, CheckpointStage},
    combat::{Action, Combat, PLAYER_RUN_SPEED},
    engine::{
        actors,
        assets::Assets,
        chapter::{
            self,
            assets::{ChapterAssets, load_world},
        },
    },
    locomotion::Gait,
    text::TextCatalog,
};
use borrow_fighters::runtime_paths::asset_path;
use raylib::prelude::*;
use std::{error::Error, path::Path};
fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(path.to_str().ok_or("path encoding")?);
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let out = Path::new(&args[1]);
    std::fs::create_dir_all(out)?;
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Borrow loading scope review")
        .hidden()
        .build();
    rl.set_target_fps(0);
    let mut target = rl.load_render_texture(&thread, 1280, 720)?;
    if args.get(2).is_some_and(|v| v == "prologue") {
        let text = TextCatalog::load(asset_path("assets/adventure/texts/pt-BR.json"))?;
        let assets = Assets::load(&mut rl, &thread, text)?;
        let mut actor = Combat::new().player;
        actor.position.x = 640.;
        actor.position.y = 620.;
        actor.action = Action::Remorse;
        for ticks in [0, 35, 65, 95, 125] {
            actor.action_ticks = ticks;
            {
                let mut d = rl.begin_texture_mode(&thread, &mut target);
                d.clear_background(Color::new(185, 193, 196, 255));
                actors::rust(&mut d, &assets, &actor, 0.);
            }
            export(&target, &out.join(format!("remorse-{ticks:03}.png")))?;
        }
    } else {
        let assets = ChapterAssets::load(&mut rl, &thread)?;
        let world = load_world()?;
        for (id, stage) in [
            ("street", CheckpointStage::StreetStart),
            ("phone", CheckpointStage::ContactReady),
            ("lane", CheckpointStage::LaneStart),
            ("passage", CheckpointStage::PassageFight),
        ] {
            let mut model = Chapter::from_checkpoint(
                world.clone(),
                Checkpoint {
                    version: 1,
                    stage,
                    prologue_played_victory: true,
                },
            )?;
            if id == "street" {
                model.combat.player.position.x += 400.;
                model.combat.player.action = Action::Walk;
                model.combat.player.gait = Gait::Run;
                model.combat.player.velocity.x = PLAYER_RUN_SPEED;
                model.combat.player.stride_distance =
                    0.5742512 * assets.common.locomotion.motion.run_stride_pixels;
            }
            if id == "phone" {
                model.phase_ticks = 150;
            }
            {
                let mut d = rl.begin_texture_mode(&thread, &mut target);
                chapter::draw(&mut d, &assets, &model, 360, false);
            }
            export(&target, &out.join(format!("chapter-{id}.png")))?;
        }
    }
    Ok(())
}
