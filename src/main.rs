extern crate core;

mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;
mod btree;
mod systems;
mod physics;
mod tasks;
mod constants;
mod world;
mod behaviors;
mod entities;
mod recipes;
mod items;
mod move_tasks;
mod item_tasks;
mod build_tasks;
mod map_util;
mod util_structs;
mod log;
mod resources;
mod textures;
mod properties;
mod needs;
mod map_generator;
mod noise_generator;
mod path;

use std::time::{Duration, Instant};
use crate::input_handler::InputHandler;
use crate::properties::Properties;
use crate::renderer::Renderer;
use crate::world::World;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let renderer = Renderer::new(&sdl_context);
    let input_handler = InputHandler::new(&sdl_context);
    let texture_creator = renderer.sdl_canvas.texture_creator();

    let mut world = World::new(renderer, input_handler, &texture_creator);
    world.setup();

    let mut instant = Instant::now();
    while !world.properties.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue; }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as f32 / 1000.0)
    }
}
