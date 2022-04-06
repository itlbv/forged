use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

use crate::components::{Position, RenderShape};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;
use crate::{entities, noise_generator, path, World};
use crate::noise_generator::Noise;

pub fn place_trees(world: &World) {
    let mut noise = noise_generator::fbm(MAP_WIDTH as usize, MAP_HEIGHT as usize, 4);
    // plant_trees(&noise_map, world);
    render_noise(&noise, world);
    draw_river(&noise, world);
}

fn draw_river(noise_map: &Noise, world: &World) {
    let (start_x, start_y) = (100, 100);
    let (finish_x, finish_y) = (0, 0);
    let path = path::dijkstra(noise_map, (start_x, start_y), (finish_x, finish_y));

    for node in path {
        let id = world.ecs.create_entity();
        world.ecs.add_component_to_entity(id, Position::of(node.0 as f32, node.1 as f32, id));
        world.ecs.add_component_to_entity(id, RenderShape::new_without_offset(1.0, 1.0,
                                                                              Color::new(200, 200, 0, 255)));
    }
}

fn plant_trees(noise_map: &Noise, world: &World) {
    let (width, height) = noise_map.size();
    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) > 0.5 {
                entities::tree(x as i32, y as i32, world);
            }
        }
    }
}

fn render_noise(noise_map: &Noise, world: &World) {
    let (width, height) = noise_map.size();
    for y in 0..height {
        for x in 0..width {
            let v = (noise_map.get_value(x, y) * 255.0).ceil() as u8;
            let id = world.ecs.create_entity();
            world.ecs.add_component_to_entity(id, Position::of(x as f32, y as f32, id));
            world.ecs.add_component_to_entity(id, RenderShape::new_without_offset(1.0, 1.0,
                                                                                  Color::new(v, v, v, 255)));
        }
    }
}
