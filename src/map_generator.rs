use rand::prelude::*;
use crate::components::{Position, RenderShape};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;
use crate::World;

type NoiseArray = [[f32; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

pub fn place_trees(world: &World) {
    let white_noise = white_noise();
    let smooth_noise = smooth_noise(&white_noise, 1);

    render_noise(&smooth_noise, world);
}

fn white_noise() -> NoiseArray {
    let mut noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    for y in 0..noise[0].len() {
        for x in 0..noise.len() {
            noise[x][y] = (rand::random::<f32>() * 255.0).ceil();
        }
    }
    noise
}

fn smooth_noise(noise: &NoiseArray, octave: usize) -> NoiseArray {
    let mut smooth_noise = [[0.0; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let sample_period = 1 << octave;
    let sample_frequency = 1.0 / sample_period as f32;

    for y in 0..noise[0].len() {
        //calculate the horizontal sampling indices
        let sample_y_0 = (y / sample_period) * sample_period;
        let sample_y_1 = (sample_y_0 + sample_period) / noise[0].len();
        let horizontal_blend = (y - sample_y_0) as f32 * sample_frequency;

        for x in 0..noise.len() {
            //calculate the vertical sampling indices
            let sample_x_0 = (x / sample_period) * sample_period;
            let sample_x_1 = (sample_x_0 + sample_period) / noise.len();
            let vertical_blend = (x - sample_x_0) as f32 * sample_frequency;

            //blend the top two corners
            let top = interpolate(noise[sample_x_0][sample_y_0],
                                  noise[sample_x_1][sample_y_0], horizontal_blend);

            //blend the bottom two corners
            let bottom = interpolate(noise[sample_x_0][sample_y_1],
                                     noise[sample_x_1][sample_y_1], horizontal_blend);
            //final blend
            smooth_noise[x][y] = interpolate(top, bottom, vertical_blend);
        }
    }
    smooth_noise
}

fn interpolate(x0: f32, x1: f32, alpha: f32) -> f32 {
    x0 * (1.0 - alpha) + alpha * x1
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
