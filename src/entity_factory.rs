use crate::components::{Behavior, Color, Food, Inventory, Name, Position, RenderShape};
use crate::{behavior_factory, World};
use crate::items::{Item, Stone, Wood};

pub fn create_house(x: f32, y: f32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 5.0, h: 3.0, color: Color { r: 100, g: 100, b: 100 } }); // grey
}

pub fn create_tree(x: f32, y: f32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 0.25, h: 0.25, color: Color { r: 100, g: 90, b: 5 } }); // brown
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Wood>(new_entity_id, Wood {});
}

pub fn create_stone(x: f32, y: f32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 0.25, h: 0.25, color: Color { r: 130, g: 130, b: 130 } }); // brown
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Stone>(new_entity_id, Stone {});
}

pub fn create_food(x: f32, y: f32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 0.3, h: 0.3, color: Color { r: 90, g: 170, b: 0 } }); // red
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Food>(new_entity_id, Food {});
}

pub fn create_mob(x: f32, y: f32, name: &str, world: &World) {
    let new_entity_id = world.ecs.create_entity();

    let behavior = Behavior { behavior_tree: Box::new(behavior_factory::build_house_sequence(new_entity_id)) };

    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<Name>(new_entity_id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape { w: 0.49, h: 0.49, color: Color { r: 0, g: 0, b: 150 } }); // blue
    world.ecs.add_component_to_entity::<Behavior>(new_entity_id, behavior);
    world.ecs.add_component_to_entity::<Inventory>(new_entity_id, Inventory::new());
}