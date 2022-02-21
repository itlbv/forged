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

use std::time::{Duration, Instant};
use crate::btree::{Sequence};
use crate::components::{Behavior, Color, Name, Position, RenderShape};
use crate::ecs::{Ecs};
use crate::input_handler::InputHandler;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::renderer::Renderer;
use crate::tasks::MoveTask;

pub struct World {
    pub quit: bool,
    pub delta_time: f32,
    map: Map,
    renderer: Renderer,
    input_handler: InputHandler,
    ecs: Ecs,
}

impl World {
    fn new(renderer: Renderer, input_handler: InputHandler) -> Self {
        Self {
            delta_time: 0.0,
            quit: false,
            map: Map::new(),
            renderer,
            input_handler,
            ecs: Ecs::new(),
        }
    }

    fn tick(&mut self, delta_time: f32) {
        self.delta_time = delta_time;

        systems::behavior(self);

        self.renderer.clear_frame();
        systems::render_map(self);
        systems::render_entities(self);
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

    setup_world(&mut world);

    let mut instant = Instant::now();
    while !world.quit {
        let frame_time = Instant::now() - instant;
        if frame_time < Duration::from_millis(16) { continue; }
        instant = Instant::now();
        world.tick(frame_time.as_millis() as f32 / 1000.0)
    }
}

fn setup_world(world: &mut World) {
    world.ecs.register_component::<Position>();
    world.ecs.register_component::<Name>();
    world.ecs.register_component::<RenderShape>();
    world.ecs.register_component::<Behavior>();

    create_mob(world, 1.5, 1.5, "Alice");
    // create_mob(world, 2.0, 2.0, "Bob");
    // create_mob(world, 3.0, 3.0, "Jim");
    // create_mob(world, 4.0, 4.0, "Karen");
}

fn create_mob(world: &mut World, x: f32, y: f32, name: &str) {
    let new_mob_id = world.ecs.create_entity();

    let sequence = Sequence {
        children: vec![
            Box::new(MoveTask::new(new_mob_id, 3.5, 8.5)),
        ]
    };
    let behavior = Behavior { behavior_tree: Box::new(sequence) };

    world.ecs.add_component_to_entity_mut::<Position>(new_mob_id, Position { x, y });
    world.ecs.add_component_to_entity_mut::<Name>(new_mob_id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(new_mob_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } });
    world.ecs.add_component_to_entity_mut::<Behavior>(new_mob_id, behavior);
}
