use crate::map::Map;
use crate::{entity_factory, InputHandler, Renderer, systems};
use crate::components::{Behavior, Food, Inventory, Name, Position, Recipe, Remove, RenderShape, Storage, Target, MainTarget, Destination, Building};
use crate::ecs::Ecs;
use crate::items::{Item, Stone, Wood};

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
        self.ecs.register_component::<Target>();
        self.ecs.register_component::<Destination>();
        self.ecs.register_component::<MainTarget>();
        self.ecs.register_component::<Item>();
        self.ecs.register_component::<Wood>();
        self.ecs.register_component::<Stone>();
        self.ecs.register_component::<Inventory>();
        self.ecs.register_component::<Storage>();
        self.ecs.register_component::<Recipe>();
        self.ecs.register_component::<Building>();

        entity_factory::create_mob(1.5, 1.5, "Alice", self);

        entity_factory::create_food(5, 8, self);
        entity_factory::create_food(4, 1, self);
        entity_factory::create_food(2, 5, self);
        entity_factory::create_food(9, 6, self);
        entity_factory::create_food(6, 6, self);
        entity_factory::create_food(5, 7, self);

        entity_factory::create_tree(3, 4, self);
        entity_factory::create_tree(7, 1, self);
        entity_factory::create_tree(8, 2, self);
        entity_factory::create_tree(1, 3, self);
        entity_factory::create_tree(3, 2, self);
        entity_factory::create_tree(5, 3, self);
        entity_factory::create_tree(6, 2, self);

        entity_factory::create_stone(13, 6, self);
        entity_factory::create_stone(14, 9, self);
        entity_factory::create_stone(12, 7, self);
    }

    pub fn tick(&mut self, _delta_time: f32) {
        self.delta_time = 0.016; // fixed framerate for debugging

        systems::behavior(self);
        systems::remove_entities(self);

        self.renderer.clear_frame();
        systems::render_map(self);
        systems::render_entities(self);
        self.renderer.present_frame();

        self.quit = self.input_handler.update();
    }
}