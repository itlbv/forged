use noise::{Fbm, MultiFractal, OpenSimplex, Perlin, Seedable, SuperSimplex};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

use crate::components::{Position, RenderShape};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;
use crate::{entities, World};

type NoiseArray = [[f32; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

pub fn place_trees(world: &World) {
    let noise = SuperSimplex::new();
        // .set_seed(0)
        // .set_octaves(16);
    let mut noise_map = PlaneMapBuilder::new(&noise)
        .set_size(MAP_WIDTH as usize, MAP_HEIGHT as usize)
        .set_x_bounds(-10.0, 10.0)
        .set_y_bounds(-10.0, 10.0)
        .build();

    normalize_noise_map(&mut noise_map);

    // plant_trees(&noise_map, world);
    render_noise_map(&noise_map, world);
}

fn normalize_noise_map(noise_map: &mut NoiseMap) {
    let (width, height) = noise_map.size();

    let mut min = 0.0;
    let mut max = 0.0;
    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) < min { min = noise_map.get_value(x, y);}
            if noise_map.get_value(x, y) > max { max = noise_map.get_value(x, y);}
        }
    }
    let factor = 1.0 / (max - min);

    for y in 0..height {
        for x in 0..width {
            let v = noise_map.get_value(x, y);
            noise_map.set_value(x, y, (v - min) * factor);
        }
    }
}

fn plant_trees(noise_map: &NoiseMap, world: &World) {
    let (width, height) = noise_map.size();
    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) > 0.5 {
                entities::tree(x as i32, y as i32, world);

            }
        }
    }
}

fn render_noise_map(noise_map: &NoiseMap, world: &World) {
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
