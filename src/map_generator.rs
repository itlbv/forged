use rand::prelude::*;
use crate::components::{Position, RenderShape};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;
use crate::World;

type NoiseArray = [[f32; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

pub fn place_trees(world: &World) {
    let mut noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    add_white_noise(&mut noise);
    render_noise(&noise, world);
}

fn add_white_noise(noise: &mut NoiseArray) {
    for y in 0..noise[0].len() {
        let noise_z = noise[0].len();
        for x in 0..noise.len() {
            noise[x][y] = (rand::random::<f32>() * 255.0).ceil();
        }
    }
}

fn render_noise(noise: &NoiseArray, world: &World) {
    for y in 0..noise[0].len() {
        for x in 0..noise.len() {
            let v = noise[x][y] as u8;
            let id = world.ecs.create_entity();
            world.ecs.add_component_to_entity(id, Position::of(x as f32, y as f32, id));
            world.ecs.add_component_to_entity(id, RenderShape::new_without_offset(1.0, 1.0,
                                                                                  Color::new(v, v, v, 255)));
        }
    }
}
