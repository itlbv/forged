use crate::map::Map;
use crate::{InputHandler, Renderer, systems};
use crate::btree::Sequence;
use crate::components::{Behavior, Color, Name, Position, RenderShape};
use crate::ecs::Ecs;
use crate::tasks::MoveTask;

pub struct World {
    pub quit: bool,
    pub delta_time: f32,
    pub map: Map,
    pub renderer: Renderer,
    input_handler: InputHandler,
    pub ecs: Ecs,
}

impl World {
    pub fn new(renderer: Renderer, input_handler: InputHandler) -> Self {
        Self {
            delta_time: 0.0,
            quit: false,
            map: Map::new(),
            renderer,
            input_handler,
            ecs: Ecs::new(),
        }
    }

    pub fn setup(&mut self) {
        self.ecs.register_component::<Position>();
        self.ecs.register_component::<Name>();
        self.ecs.register_component::<RenderShape>();
        self.ecs.register_component::<Behavior>();

        self.create_mob(1.5, 1.5, "Alice");
        // create_mob(world, 2.0, 2.0, "Bob");
        // create_mob(world, 3.0, 3.0, "Jim");
        // create_mob(world, 4.0, 4.0, "Karen");
    }

    fn create_mob(&mut self, x: f32, y: f32, name: &str) {
        let new_mob_id = self.ecs.create_entity();

        let sequence = Sequence {
            children: vec![
                Box::new(MoveTask::new(new_mob_id, 3.5, 8.5)),
            ]
        };
        let behavior = Behavior { behavior_tree: Box::new(sequence) };

        self.ecs.add_component_to_entity_mut::<Position>(new_mob_id, Position { x, y });
        self.ecs.add_component_to_entity_mut::<Name>(new_mob_id, Name { v: name.to_string() });
        self.ecs.add_component_to_entity_mut::<RenderShape>(new_mob_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } });
        self.ecs.add_component_to_entity_mut::<Behavior>(new_mob_id, behavior);
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.delta_time = delta_time;

        systems::behavior(self);

        self.renderer.clear_frame();
        systems::render_map(self);
        systems::render_entities(self);
        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}