//! Draws neighbours entering the shop, its descending shutter and the caramelo.
//!
//! System: Adventure presentation. Catalog pieces share the street's world
//! transform; geometric doorway crops remain correct throughout camera motion.

use raylib::prelude::*;

use super::{assets::Assets, pieces::PiecePose};
use crate::adventure::{
    ambient::AmbientState,
    combat::Facing,
    neighborhood::{
        DOOR_CENTER_X, DOOR_HEIGHT, DOOR_WIDTH, DogPhase, ENTERING_TICKS, NEIGHBOR_FLOOR_Y,
        Neighborhood, ResidentPhase, SHOPKEEPER_ID, ShutterPhase,
    },
};

/// Draws the shop's occupants and sidewalk neighbours after props, before cars.
pub fn draw(d: &mut impl RaylibDraw, ambient: &AmbientState, assets: &Assets, offset: f32) {
    let neighborhood = Neighborhood::sample(ambient);
    let door = Rectangle::new(
        DOOR_CENTER_X - DOOR_WIDTH * 0.5 - offset,
        NEIGHBOR_FLOOR_Y - DOOR_HEIGHT,
        DOOR_WIDTH,
        DOOR_HEIGHT,
    );
    // The painted open door remains visible through a dark interior veil.
    // Entrants are clipped to this opening as they move behind the left jamb.
    d.draw_rectangle_rec(door, Color::new(14, 24, 20, 135));
    for resident in neighborhood
        .residents
        .iter()
        .filter(|resident| resident.visible)
    {
        let inside = resident.id == SHOPKEEPER_ID || resident.phase == ResidentPhase::Entering;
        let (id, ticks, flip) = if resident.id == SHOPKEEPER_ID {
            if resident.phase == ResidentPhase::Closing {
                ("shopkeeper.pull", resident.phase_ticks, false)
            } else if resident.phase == ResidentPhase::Waiting {
                ("shopkeeper.alert", resident.phase_ticks, false)
            } else {
                ("shopkeeper.idle", resident.phase_ticks, false)
            }
        } else {
            let moving = matches!(
                resident.phase,
                ResidentPhase::Running | ResidentPhase::Entering
            );
            let id = match (resident.id == 1, moving) {
                (false, false) => "resident.0.idle",
                (false, true) => "resident.0.run",
                (true, false) => "resident.1.idle",
                (true, true) => "resident.1.run",
            };
            (
                id,
                resident.phase_ticks,
                moving && resident.facing == Facing::Right,
            )
        };
        let mut pose = PiecePose::at(Vector2::new(
            resident.position.x - offset,
            resident.position.y,
        ));
        pose.flip = flip;
        if resident.id == 2 {
            // Reusing an existing resident piece keeps the street modular.
            pose.scale = 0.94;
            pose.tint = Color::new(224, 233, 217, 255);
        }
        if resident.phase == ResidentPhase::Entering {
            let progress = resident.phase_ticks as f32 / ENTERING_TICKS as f32;
            let shade = (230.0 - 48.0 * progress) as u8;
            pose.tint = Color::new(shade, shade, shade, 255);
        }
        if inside {
            assets.street.draw_clipped(d, id, ticks, &pose, door);
        } else {
            d.draw_ellipse(
                pose.position.x as i32,
                pose.position.y as i32,
                16.0,
                2.5,
                Color::new(37, 42, 31, 42),
            );
            assets.street.draw(d, id, ticks, &pose);
        }
    }

    let dog = neighborhood.dog;
    if dog.visible {
        let (id, ticks) = match dog.phase {
            DogPhase::Idle => ("dog.idle", dog.phase_ticks),
            DogPhase::Sitting => ("dog.sit", dog.phase_ticks),
            DogPhase::Sniffing => ("dog.sniff", dog.phase_ticks),
            DogPhase::Wandering => ("dog.run", dog.phase_ticks / 3),
            DogPhase::Startled => ("dog.alert", dog.phase_ticks),
            DogPhase::Running => ("dog.run", dog.phase_ticks),
            DogPhase::Gone => unreachable!("An escaped dog must not be visible"),
        };
        let mut pose = PiecePose::at(Vector2::new(dog.position.x - offset, dog.position.y));
        pose.flip = dog.facing == Facing::Left;
        d.draw_ellipse(
            pose.position.x as i32,
            pose.position.y as i32,
            25.0,
            2.5,
            Color::new(37, 42, 31, 42),
        );
        assets.street.draw(d, id, ticks, &pose);
    }

    if neighborhood.shutter.phase != ShutterPhase::Open {
        // Translate the full panel downward, exposing only its lower section.
        // The bottom rail/handle follows the contact edge without scaling rows.
        let bottom = door.y + door.height * neighborhood.shutter.progress;
        let pose = PiecePose::at(Vector2::new(DOOR_CENTER_X - offset, bottom));
        assets
            .street
            .draw_clipped(d, "shop.shutter", 0, &pose, door);
        d.draw_line_ex(
            Vector2::new(door.x, bottom),
            Vector2::new(door.x + door.width, bottom),
            1.8,
            Color::new(49, 48, 41, 230),
        );
        if neighborhood.shutter.phase == ShutterPhase::Closing {
            // A short pull strap links the high/low authored hand poses to the
            // rolling edge. Once the edge covers the hands it covers the strap too.
            let grip_y = NEIGHBOR_FLOOR_Y
                - if neighborhood.shutter.progress < 0.5 {
                    99.0
                } else {
                    44.0
                };
            if grip_y > bottom {
                let center = DOOR_CENTER_X - offset;
                d.draw_line_ex(
                    Vector2::new(center, bottom),
                    Vector2::new(center, grip_y),
                    1.8,
                    Color::new(75, 67, 49, 255),
                );
                d.draw_line_ex(
                    Vector2::new(center - 10.0, grip_y),
                    Vector2::new(center + 10.0, grip_y),
                    2.1,
                    Color::new(105, 105, 88, 255),
                );
            }
        }
    }
}
