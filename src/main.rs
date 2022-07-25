extern crate core;

mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;
mod systems;
mod constants;
mod world;
mod entities;
mod recipes;
mod items;
mod asset_manager;
mod textures;
mod properties;
mod util;
mod behavior;
mod actions;


use std::time::{Duration, Instant};
use sdl2::render::BlendMode::Blend;
use crate::constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use crate::input_handler::InputHandler;
use crate::properties::Properties;
use crate::renderer::Renderer;
use crate::world::World;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video = sdl_context.video()?;
    let sdl_window = sdl_video
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build().map_err(|e| e.to_string())?;

    let mut sdl_canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().map_err(|e| e.to_string())?;
    sdl_canvas.set_blend_mode(Blend);

    // let sdl_texture_creator = sdl_canvas.texture_creator();
    // let sdl_ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;


    let renderer = Renderer::new(sdl_canvas);

    let sdl_event_pump = sdl_context.event_pump()?;
    let input_handler = InputHandler::new(sdl_event_pump);

    let texture_creator = renderer.sdl_canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut world = World::new(renderer, input_handler, &texture_creator, &ttf_context);
    world.setup();

    let mut instant = Instant::now();
    while !world.properties.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue; }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as f32 / 1000.0)
    }

    Ok(())
}
