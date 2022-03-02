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
mod behavior_factory;
mod entity_factory;
mod recipes;
mod items;
mod move_tasks;
mod item_tasks;
mod build_tasks;

use std::time::{Duration, Instant};
use crate::input_handler::InputHandler;
use crate::renderer::Renderer;
use crate::world::World;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Forged", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let sdl_canvas = window.into_canvas().build().unwrap();
    let renderer = Renderer::new(sdl_canvas);

    let sdl_events = sdl_context.event_pump().unwrap();
    let input_handler = InputHandler { sdl_events };

    let mut world = World::new(renderer, input_handler);
    world.setup();

    let mut instant = Instant::now();
    while !world.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue; }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as f32 / 1000.0)
    }
}
