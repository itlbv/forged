mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;

use std::time::{Duration, Instant};
use crate::components::{Name, Position, RenderShape};
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
        self.renderer.render_map(&mut self.map);
        render_sys(self);
        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}

fn render_sys(world: &mut World) {
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
        world.renderer.render_rect(x, y, w, h);
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

    let entity_1 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Position>(entity_1, Position { x: 1.0, y: 1.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_1, Name { v: "Alice".to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(entity_1, RenderShape {w: 0.49, h: 0.49});

    let entity_2 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Position>(entity_2, Position { x: 2.0, y: 2.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_2, Name { v: "Bob".to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(entity_2, RenderShape {w: 0.49, h: 0.49});

    let entity_3 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Position>(entity_3, Position { x: 3.0, y: 3.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_3, Name { v: "Entity 3".to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(entity_3, RenderShape {w: 0.49, h: 0.49});

    let entity_4 = world.ecs.create_entity();
    world.ecs.add_component_to_entity_mut::<Position>(entity_4, Position { x: 4.0, y: 4.0 });
    world.ecs.add_component_to_entity_mut::<Name>(entity_4, Name { v: "Entity 4".to_string() });
    world.ecs.add_component_to_entity_mut::<RenderShape>(entity_4, RenderShape {w: 0.49, h: 0.49});

}
