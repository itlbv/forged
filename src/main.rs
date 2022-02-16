mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;

use std::borrow::BorrowMut;
use std::time::{Duration, Instant};
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
    world.ecs.add_component_to_entity::<Pos>(entity_1, Pos { x: 1.0, y: 1.0 });
    world.ecs.add_component_to_entity::<Name>(entity_1, Name { name: "Alice".to_string() });
    world.ecs.add_component_to_entity::<Health>(entity_1, Health { health: 5 });

    let entity_2 = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Pos>(entity_2, Pos { x: 2.0, y: 2.0 });
    world.ecs.add_component_to_entity::<Name>(entity_2, Name { name: "Bob".to_string() });
    world.ecs.add_component_to_entity::<Health>(entity_2, Health { health: 5 });

    let entity_3 = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Pos>(entity_3, Pos { x: 3.0, y: 3.0 });
    world.ecs.add_component_to_entity::<Name>(entity_3, Name { name: "No health".to_string() });

    let entity_4 = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Pos>(entity_4, Pos { x: 1.0, y: 1.0 });

    {
        let mut pos = world.ecs.borrow::<Pos>().unwrap();
        let mut name = world.ecs.borrow::<Name>().unwrap();
        let zip = pos.iter_mut().zip(name.iter_mut());
        let iter = zip.filter_map(|(pos, name)| Some((pos.as_mut()?, name.as_mut()?)));
        for (pos, name) in iter {
            println!("{}, {}", pos.x, name.name);
        }
    };

    {
        let mut pos = world.ecs.borrow_vec_m::<Pos>();
        let mut name = world.ecs.borrow_vec_m::<Name>();
        let zip = pos.iter_mut().zip(name.iter_mut());
        let iter = zip.filter_map(|(pos, name)| Some((pos.as_mut()?, name.as_mut()?)));
        for (pos, name) in iter {
            println!("{}, {}", pos.x, name.name);
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
