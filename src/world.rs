use crate::map::Map;
use crate::{behavior_factory, InputHandler, Renderer, systems};
use crate::components::{Behavior, Color, Food, Name, Position, Remove, RenderShape, TargetEntity, TargetPosition};
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

        self.create_mob(1.5, 1.5, "Alice");

        self.create_food(5.5, 8.5);
        self.create_food(4.5, 1.5);
        self.create_food(2.5, 4.5);
        self.create_food(9.5, 6.5);
    }

    fn create_mob(&mut self, x: f32, y: f32, name: &str) {
        let new_mob_id = self.ecs.create_entity();

        let behavior = Behavior { behavior_tree: Box::new(behavior_factory::find_food_sequence(new_mob_id)) };

        self.ecs.add_component_to_entity_mut::<Position>(new_mob_id, Position::of(x, y, new_mob_id));
        self.ecs.add_component_to_entity_mut::<Name>(new_mob_id, Name { v: name.to_string() });
        self.ecs.add_component_to_entity_mut::<RenderShape>(new_mob_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } });
        self.ecs.add_component_to_entity_mut::<Behavior>(new_mob_id, behavior);
    }

    fn create_food(&mut self, x: f32, y: f32) {
        let new_entity_id = self.ecs.create_entity();
        self.ecs.add_component_to_entity_mut::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
        self.ecs.add_component_to_entity_mut::<RenderShape>(new_entity_id, RenderShape { w: 0.2, h: 0.2, color: Color { r: 0, g: 0, b: 150 } });
        self.ecs.add_component_to_entity_mut::<Food>(new_entity_id, Food{});
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