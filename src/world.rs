use crate::map::Map;
use crate::{entity_factory, InputHandler, Renderer, systems};
use crate::components::{Behavior, Food, Item, Name, Position, Remove, RenderShape, TargetEntity, TargetPosition};
use crate::ecs::Ecs;

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
        self.ecs.register_component::<Food>();
        self.ecs.register_component::<Remove>();
        self.ecs.register_component::<TargetEntity>();
        self.ecs.register_component::<TargetPosition>();
        self.ecs.register_component::<Item>();

        entity_factory::create_mob(1.5, 1.5, "Alice", self);

        entity_factory::create_food(5.5, 8.5, self);
        entity_factory::create_food(4.5, 1.5, self);
        entity_factory::create_food(2.5, 4.5, self);
        entity_factory::create_food(9.5, 6.5, self);
        entity_factory::create_food(6.5, 6.5, self);
        entity_factory::create_food(5.5, 7.5, self);

        entity_factory::create_tree(3.5, 4.5, self);
        entity_factory::create_tree(7.5, 1.5, self);
        entity_factory::create_tree(8.5, 2.5, self);
        entity_factory::create_tree(1.5, 3.5, self);
        entity_factory::create_tree(3.5, 2.5, self);
        entity_factory::create_tree(5.5, 3.5, self);
        entity_factory::create_tree(6.5, 2.5, self);
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.delta_time = delta_time;

        systems::behavior(self);
        systems::remove_entities(self);

        self.renderer.clear_frame();
        systems::render_map(self);
        systems::render_entities(self);
        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}