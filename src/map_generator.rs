use noise::{Fbm, MultiFractal, OpenSimplex, Perlin, Seedable};
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
    let noise = Fbm::new()
        .set_seed(0)
        .set_octaves(16);
    let noise_map = PlaneMapBuilder::new(&noise)
        .set_size(MAP_WIDTH as usize, MAP_HEIGHT as usize)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build();

    plant_trees(&noise_map, world);
    // render_noise_map(&noise_map, world);
}

fn plant_trees(noise_map: &NoiseMap, world: &World) {
    let (width, height) = noise_map.size();
    let mut min = 0.0;
    let mut max = 0.0;
    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) < min { min = noise_map.get_value(x, y);}
            if noise_map.get_value(x, y) > max { max = noise_map.get_value(x, y);}
        }
    }
    let median = min + ((max - min) / 2.0);
    println!("{}, {}, {}", max, min, median);

    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) > median {
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

fn _white_noise() -> NoiseArray {
    let mut rng = ChaCha8Rng::seed_from_u64(2);

    let mut noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let height = noise[0].len();
    let width = noise.len();
    for y in 0..height {
        for x in 0..width {
            noise[x][y] = (rng.gen::<f32>() * 255.0).ceil();
        }
    }
    noise
}

fn _add_perlin_noise(base_noise: &NoiseArray) -> NoiseArray {
    const OCTAVE_COUNT: usize = 8;
    let persistence = 0.8;

    //generate smooth noises
    let mut smooth_noises = [[[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize]; OCTAVE_COUNT];
    for i in 0..OCTAVE_COUNT {
        smooth_noises[i] = _add_smooth_noise(base_noise, i);
    }

    // float[][] perlinNoise = GetEmptyArray<float>(width, height); //an array of floats initialised to 0
    let mut perlin_noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // float amplitude = 1.0f;
    let mut amplitude = 1.0;
    // float totalAmplitude = 0.0f;
    let mut total_amplitude = 0.0;

    let height = base_noise[0].len();
    let width = base_noise.len();

    //blend noise together
    let mut octave = OCTAVE_COUNT - 1;
    while octave >= 0 {
        amplitude *= persistence;
        total_amplitude += amplitude;

        for y in 0..height {
            for x in 0..width {
                perlin_noise[x][y] += smooth_noises[octave][x][y] * amplitude;
            }
        }

        octave -= 1;
        if octave == 0 { break; }
    }

    //normalisation
    for y in 0..height {
        for x in 0..width {
            perlin_noise[x][y] /= total_amplitude;
        }
    }
    perlin_noise
}

fn _add_smooth_noise(base_noise: &NoiseArray, octave: usize) -> NoiseArray {
    let mut smooth_noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let sample_period = 1 << octave;
    let sample_frequency = 1.0 / sample_period as f32;

    let height = base_noise[0].len();
    let width = base_noise.len();

    for y in 0..height {
        //calculate the vertical sampling indices
        let sample_y_0 = (y / sample_period) * sample_period;
        let sample_y_1 = (sample_y_0 + sample_period) / height;
        let vertical_blend = (y - sample_y_0) as f32 * sample_frequency;

        for x in 0..width {
            //calculate the horizontal sampling indices
            let sample_x_0 = (x / sample_period) * sample_period;
            let sample_x_1 = (sample_x_0 + sample_period) / width;
            let horizontal_blend = (x - sample_x_0) as f32 * sample_frequency;

            //blend the top two corners
            let top = _interpolate(base_noise[sample_x_0][sample_y_0],
                                   base_noise[sample_x_1][sample_y_0],
                                   horizontal_blend);

            //blend the bottom two corners
            let bottom = _interpolate(base_noise[sample_x_0][sample_y_1],
                                      base_noise[sample_x_1][sample_y_1],
                                      horizontal_blend);
            //final blend
            smooth_noise[x][y] = _interpolate(top, bottom, vertical_blend);
        }
    }
    smooth_noise
}

fn _render_noise(noise: &NoiseArray, world: &World) {
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

fn _interpolate(x0: f32, x1: f32, alpha: f32) -> f32 {
    x0 * (1.0 - alpha) + alpha * x1
}
