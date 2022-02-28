use crate::components::{Behavior, Color, Food, Name, Position, RenderShape};
use crate::{behavior_factory, World};

pub fn create_house(x: f32, y: f32, world: &World) {
    let house_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(house_id, Position::of(x, y, house_id));
    world.ecs.add_component_to_entity::<RenderShape>(house_id, RenderShape { w: 5.0, h: 3.0, color: Color { r: 0, g: 0, b: 150 } });
}

pub fn create_food(x: f32, y: f32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 0.2, h: 0.2, color: Color { r: 0, g: 0, b: 150 } });
    world.ecs.add_component_to_entity::<Food>(new_entity_id, Food{});
}

pub fn create_mob(x: f32, y: f32, name: &str, world: &World) {
    let new_mob_id = world.ecs.create_entity();

    let behavior = Behavior { behavior_tree: Box::new(behavior_factory::build_house_sequence(new_mob_id)) };

    world.ecs.add_component_to_entity::<Position>(new_mob_id, Position::of(x, y, new_mob_id));
    world.ecs.add_component_to_entity::<Name>(new_mob_id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity::<RenderShape>(new_mob_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } });
    world.ecs.add_component_to_entity::<Behavior>(new_mob_id, behavior);
}