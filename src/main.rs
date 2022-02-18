mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;

use std::time::{Duration, Instant};
use crate::components::{Color, Name, Position, RenderShape};
use crate::ecs::{Ecs};
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

        render_map_sys(self);
        render_entities_sys(self);

        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}

fn render_map_sys(world: &mut World) {
    for map_node in &world.map.nodes {
        let x = Renderer::world_to_screen(map_node.x as f32);
        let y = Renderer::world_to_screen(map_node.y as f32);
        let w = Renderer::world_to_screen(1.0);
        let h = Renderer::world_to_screen(1.0);
        world.renderer.render_rect(x - w / 2, y - h / 2, w, h, map_node.color.r, map_node.color.g, map_node.color.b);
        world.renderer.render_dot(x, y); //render true position
    }
}

fn render_entities_sys(world: &mut World) {
    let mut shapes = world.ecs.borrow_component_vec_mut::<RenderShape>();
    let mut positions = world.ecs.borrow_component_vec_mut::<Position>();

    let zip = shapes.iter_mut().zip(positions.iter_mut());
    let iter = zip.filter_map(
        |(shape, pos)| Some((shape.as_ref()?, pos.as_ref()?))
    );

    for (shape, pos) in iter {
        let x = Renderer::world_to_screen(pos.x);
        let y = Renderer::world_to_screen(pos.y);
        let w = Renderer::world_to_screen(shape.w);
        let h = Renderer::world_to_screen(shape.h);
        world.renderer.render_rect(x - w / 2, y - h / 2, w, h, shape.color.r, shape.color.g, shape.color.b);
        world.renderer.render_dot(x, y); //render true position
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
        world.tick(frame_time.as_millis() as i32)
    }
}

fn setup_world(world: &mut World) {
    world.ecs.register_component::<Position>();
    world.ecs.register_component::<Name>();
    world.ecs.register_component::<RenderShape>();

    create_mob(world, 1.0, 1.0, "Alice");
    create_mob(world, 2.0, 2.0, "Bob");
    create_mob(world, 3.0, 3.0, "Jim");
    create_mob(world, 4.0, 4.0, "Karen");
}

fn create_mob(world: &mut World, x: f32, y: f32, name: &str) {
    let new_mob_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Position>(new_mob_id, Position { x, y });
    world.ecs.add_component_to_entity_mut::<Name>(new_mob_id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(new_mob_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } });
}
