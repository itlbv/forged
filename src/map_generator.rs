use rand::prelude::*;
use crate::components::{Position, RenderShape};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, WINDOW_WIDTH};
use crate::util_structs::Color;

use crate::World;

pub fn place_trees(world: &World) {
    const W: usize = MAP_WIDTH as usize;
    const H: usize = MAP_HEIGHT as usize;
    let noise: [[f32; W]; H] = [[0.0; W]; H];

    for y in 0..noise.len() {
        for x in 0..noise[y].len() {
            let value = (rand::random::<f32>() * 255.0).ceil();

            let id = world.ecs.create_entity();
            world.ecs.add_component_to_entity(id, Position::of(x as f32, y as f32, id));
            world.ecs.add_component_to_entity(id, RenderShape::new_without_offset(1.0, 1.0,
                                                                                  Color::new(value as u8, value as u8, value as u8, 255),
            ));
        }
    }
}