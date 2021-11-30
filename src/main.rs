mod map;
mod renderer;
mod input_handler;

use std::time::{Duration, Instant};
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use crate::input_handler::InputHandler;
use crate::map::Map;
use crate::renderer::Renderer;

pub struct World {
    pub quit: bool,
    map: Map,
    renderer: Renderer,
    input_handler: InputHandler,
}

impl World {
    fn new(renderer: Renderer, input_handler: InputHandler) -> Self {
        Self {
            quit: false,
            map: Map::new(),
            renderer,
            input_handler,
        }
    }

    fn tick(&mut self, delta_time: i32) {
        self.renderer.clear_frame();
        self.renderer.render_map(&mut self.map);
        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Forged", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut sdl_canvas = window.into_canvas().build().unwrap();
    let renderer = Renderer::new(sdl_canvas);

    let mut sdl_events = sdl_context.event_pump().unwrap();
    let input_handler = InputHandler{sdl_events};

    let mut world = World::new(renderer, input_handler);

    let mut instant = Instant::now();
    while !world.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as i32)
    }
}
