use crate::components::{Behavior, Building, Food, Inventory, Name, Position, Recipe, RenderShape, Storage};
use crate::{behaviors, textures, World};
use crate::items::{Item, Stone, Wood};
use crate::util_structs::Color;

pub fn house_from_recipe(x: f32, y: f32, recipe: Recipe, world: &World) -> (usize, f32, f32) {
    let id = world.ecs.create_entity();
    world.ecs.add_component_to_entity(id, Position::of(x, y, id));
    let entry_x = x - 0.5;
    let entry_y = y + 4.5;
    world.ecs.add_component_to_entity(id, Building::new(entry_x, entry_y));
    world.ecs.add_component_to_entity(id, Storage::new());
    world.ecs.add_component_to_entity(id, recipe.texture);
    (id, entry_x, entry_y)
}

pub fn tree(x: i32, y: i32, world: &World) {
    let id = world.ecs.create_entity();
    world.ecs.add_component_to_entity(id, Position::of(x as f32 + 0.5, y as f32 + 0.5, id));
    world.ecs.add_component_to_entity(id, RenderShape::new_with_standard_offset(0.25, 0.25,
                                                                                Color::new(100, 90, 5, 255))); // brown
    world.ecs.add_component_to_entity(id, Item {});
    world.ecs.add_component_to_entity(id, Wood {});
    world.ecs.add_component_to_entity(id, textures::tree());

    world.map.set_tile_passable(x, y, true);
}

pub fn stone(x: i32, y: i32, world: &World) {
    let id = world.ecs.create_entity();
    world.ecs.add_component_to_entity(id, Position::of(x as f32 + 0.5, y as f32 + 0.5, id));
    world.ecs.add_component_to_entity(id, RenderShape::new_with_standard_offset(0.25, 0.25,
                                                                                Color::new(130, 130, 130, 255))); // brown
    world.ecs.add_component_to_entity(id, Item {});
    world.ecs.add_component_to_entity(id, Stone {});
    world.ecs.add_component_to_entity(id, textures::stone());

    world.map.set_tile_passable(x, y, true);
}

pub fn food(x: i32, y: i32, world: &World) {
    let id = world.ecs.create_entity();
    world.ecs.add_component_to_entity(id, Position::of(x as f32 + 0.5, y as f32 + 0.5, id));
    world.ecs.add_component_to_entity(id, RenderShape::new_with_standard_offset(0.3, 0.3,
                                                                                Color::new(90, 170, 0, 255))); // red
    world.ecs.add_component_to_entity(id, Item {});
    world.ecs.add_component_to_entity(id, Food {});
    world.ecs.add_component_to_entity(id, textures::food());


    world.map.set_tile_passable(x, y, true);
}

pub fn human(x: f32, y: f32, name: &str, world: &World) {
    let id = world.ecs.create_entity();

    world.ecs.add_component_to_entity(id, Position::of(x, y, id));
    world.ecs.add_component_to_entity(id, RenderShape::new_with_standard_offset(0.49, 0.49,
                                                                                Color::new(0, 0, 150, 255))); // blue
    world.ecs.add_component_to_entity(id, Name { v: name.to_string() });
    world.ecs.add_component_to_entity(id, Behavior::new_with_initial_behavior(Box::new(behaviors::do_nothing()), id));
    world.ecs.add_component_to_entity(id, Inventory::new());
    world.ecs.add_component_to_entity(id, textures::human());
}

pub fn player(x: f32, y: f32, world: &World) -> usize {
    let id = world.ecs.create_entity();

    world.ecs.add_component_to_entity(id, Position::of(x, y, id));
    world.ecs.add_component_to_entity(id, RenderShape::new_with_standard_offset(0.49, 0.49,
                                                                                Color::new(0, 0, 150, 255))); // blue
    world.ecs.add_component_to_entity(id, textures::human());

    id
}