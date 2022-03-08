use crate::components::{Behavior, Building, Food, Inventory, Name, Position, Recipe, RenderShape, Storage};
use crate::{behaviors, World};
use crate::items::{Item, Stone, Wood};
use crate::util_structs::Color;

pub fn foundation(x: f32, y: f32, recipe: Recipe, world: &World) -> usize {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));

    let r_shape = &recipe.render_shape;
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id,
                                                     RenderShape::new_without_offset(
                                                         r_shape.w,
                                                         r_shape.h,
                                                         Color::new(140, 140, 140, 120))); // light grey

    world.ecs.add_component_to_entity::<Recipe>(new_entity_id, recipe);
    world.ecs.add_component_to_entity::<Storage>(new_entity_id, Storage::new());

    new_entity_id
}

pub fn house(x: f32, y: f32, recipe: Recipe, world: &World) -> (usize, f32, f32) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity(new_entity_id, Position::of(x, y, new_entity_id));
    let entry_x = x + recipe.render_shape.w / 2.0;
    let entry_y = y + recipe.render_shape.h + 0.5;
    world.ecs.add_component_to_entity(new_entity_id, Building::new(entry_x, entry_y));
    world.ecs.add_component_to_entity(new_entity_id, Storage::new());
    (new_entity_id, entry_x, entry_y)
}

pub fn create_tree(x: i32, y: i32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x as f32 + 0.5, y as f32 + 0.5, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape::new_with_standard_offset(0.25, 0.25, Color::new(100, 90, 5, 255))); // brown
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Wood>(new_entity_id, Wood {});

    world.map.set_node_occupied(x, y, true);
}

pub fn create_stone(x: i32, y: i32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x as f32 + 0.5, y as f32 + 0.5, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape::new_with_standard_offset(0.25, 0.25, Color::new(130, 130, 130, 255))); // brown
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Stone>(new_entity_id, Stone {});

    world.map.set_node_occupied(x, y, true);
}

pub fn create_food(x: i32, y: i32, world: &World) {
    let new_entity_id = world.ecs.create_entity();
    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x as f32 + 0.5, y as f32 + 0.5, new_entity_id));
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape::new_with_standard_offset(0.3, 0.3, Color::new(90, 170, 0, 255))); // red
    world.ecs.add_component_to_entity::<Item>(new_entity_id, Item {});
    world.ecs.add_component_to_entity::<Food>(new_entity_id, Food {});

    world.map.set_node_occupied(x, y, true);
}

pub fn create_mob(x: f32, y: f32, name: &str, world: &World) {
    let new_entity_id = world.ecs.create_entity();

    let behavior = Behavior { behavior_tree: Box::new(behaviors::build_house(new_entity_id)) };

    world.ecs.add_component_to_entity::<Position>(new_entity_id, Position::of(x, y, new_entity_id));
    world.ecs.add_component_to_entity::<Name>(new_entity_id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity::<RenderShape>(new_entity_id, RenderShape::new_with_standard_offset(0.49, 0.49, Color::new(0, 0, 150, 255))); // blue
    world.ecs.add_component_to_entity::<Behavior>(new_entity_id, behavior);
    world.ecs.add_component_to_entity::<Inventory>(new_entity_id, Inventory::new());
}