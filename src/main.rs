mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;

use std::borrow::BorrowMut;
use std::time::{Duration, Instant};
use std::iter::IntoIterator;
// use core::iter::IntoIterator;
use crate::components::{Health, Name, Pos};
use crate::ecs::Ecs;
use crate::input_handler::InputHandler;
use crate::map::Map;
use crate::renderer::Renderer;

pub struct World {
    pub quit: bool,
    map: Map,
    renderer: Renderer,
    input_handler: InputHandler,
    ecs: Ecs,
}

impl World {
    fn new(renderer: Renderer, input_handler: InputHandler) -> Self {
        Self {
            quit: false,
            map: Map::new(),
            renderer,
            input_handler,
            ecs: Ecs::new(),
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

    let sdl_canvas = window.into_canvas().build().unwrap();
    let renderer = Renderer::new(sdl_canvas);

    let sdl_events = sdl_context.event_pump().unwrap();
    let input_handler = InputHandler { sdl_events };

    let mut world = World::new(renderer, input_handler);

    world.ecs.register_component::<Pos>();
    world.ecs.register_component::<Name>();
    world.ecs.register_component::<Health>();

    let entity_1 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Pos>(entity_1, Pos { x: 1.0, y: 1.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_1, Name { name: "Alice".to_string() });
    world.ecs.add_component_to_entity_mut::<Health>(entity_1, Health { health: 5 });

    let entity_2 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Pos>(entity_2, Pos { x: 2.0, y: 2.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_2, Name { name: "Bob".to_string() });
    world.ecs.add_component_to_entity_mut::<Health>(entity_2, Health { health: 5 });

    let entity_3 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Pos>(entity_3, Pos { x: 3.0, y: 3.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_3, Name { name: "No health".to_string() });

    let entity_4 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Pos>(entity_4, Pos { x: 1.0, y: 1.0 });

    {
        let mut pos = world.ecs.borrow_component_vec_mut::<Pos>();
        let mut name = world.ecs.borrow_component_vec_mut::<Name>();
        let mut health = world.ecs.borrow_component_vec_mut::<Health>();

        for (pos, name, health) in izip!(pos.iter_mut(), name.iter_mut(), health.iter_mut()) {
            let p = pos.as_ref().unwrap();
            let n = name.as_ref().unwrap();
            let h = health.as_ref().unwrap();
            println!("x: {}, y:{}, name: {}, health: {}", p.x, p.y, n.name, h.health);
        }
    }

    let mut instant = Instant::now();
    while !world.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue; }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as i32)
    }
}
