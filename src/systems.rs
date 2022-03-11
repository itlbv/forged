use crate::components::{Behavior, Position, Remove, RenderShape};
use crate::{behaviors, Renderer, World};
use crate::btree::Status::{FAILURE, SUCCESS};
use crate::constants::{MAP_HEIGHT, MAP_TILE_SIZE, MAP_TILE_SIZE_PXL, MAP_WIDTH};
use crate::util_structs::Color;

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        let b = behavior.as_mut();
        match b {
            None => { continue; }
            Some(_) => { run_behavior_tree(b.unwrap(), world) }
        };
    }
}

fn run_behavior_tree(behavior: &mut Behavior, world: &World) {
    let status = behavior.behavior_tree.run(world);
    if status == SUCCESS || status == FAILURE {
        behavior.behavior_tree = Box::new(behaviors::do_nothing());
    }
}

pub fn remove_entities(world: &mut World) {
    let mut entity_ids_to_remove: Vec<usize> = vec![];
    {
        let removals = world.ecs.borrow_component_vec::<Remove>();
        let iter = removals.iter().filter_map(|removal| Some(removal.as_ref()?)).clone();
        for removal in iter {
            entity_ids_to_remove.push(removal.own_id);
        }
    }
    for entity_id in entity_ids_to_remove {
        world.ecs.remove_entity(entity_id);
    }
}

pub fn render_entities(world: &mut World) {
    let shapes = world.ecs.borrow_component_vec::<RenderShape>();
    let positions = world.ecs.borrow_component_vec::<Position>();

    let zip = shapes.iter().zip(positions.iter());
    let iter = zip.filter_map(
        |(shape, pos)| Some((shape.as_ref()?, pos.as_ref()?))
    );

    for (shape, pos) in iter {
        let x = Renderer::world_to_screen(pos.x + shape.offset_x);
        let y = Renderer::world_to_screen(pos.y + shape.offset_y);
        let w = Renderer::world_to_screen(shape.w);
        let h = Renderer::world_to_screen(shape.h);
        //world.renderer.render_rect(x - w / 2, y - h / 2, w, h, shape.color.r, shape.color.g, shape.color.b, shape.color.a);
        world.renderer.render_rect(x, y, w, h, shape.color.r, shape.color.g, shape.color.b, shape.color.a);
        let true_pos_x = Renderer::world_to_screen(pos.x);
        let true_pos_y = Renderer::world_to_screen(pos.y);
        world.renderer.render_dot(true_pos_x, true_pos_y); // true position
    }
}

pub fn render_map(world: &mut World) {
    for tile in world.map.borrow_tiles().iterator() {
        let x = Renderer::world_to_screen(tile.x as f32);
        let y = Renderer::world_to_screen(tile.y as f32);
        let tile_size = Renderer::world_to_screen(MAP_TILE_SIZE);

        let color: Color;
        if tile.occupied {
            color = Color::new(10, 10, 10, 255);
        } else {
            color = Color::new(tile.color.r, tile.color.g, tile.color.b, tile.color.a);
        };

        world.renderer.render_rect(
            x,
            y,
            tile_size,
            tile_size,
            color.r, color.g, color.b, color.a);
        // tile.color.r, tile.color.g, tile.color.b, tile.color.a);
        world.renderer.render_dot(x, y); // true position

        world.renderer.render_texture(
            world.assets.borrow_texture("map_tileset"),
            tile.tileset_x, tile.tileset_y, tile.tileset_w, tile.tileset_h,
            x, y, MAP_TILE_SIZE_PXL, MAP_TILE_SIZE_PXL);
    }

    for x in 0..=MAP_WIDTH { // vertical lines
        let x_1: i32 = 50 * x * MAP_TILE_SIZE as i32;
        let y_1: i32 = 0;
        let x_2: i32 = x_1;
        let y_2: i32 = 50 * MAP_HEIGHT * MAP_TILE_SIZE as i32;
        // world.renderer.render_line(x_1, y_1, x_2, y_2);
    }

    for y in 0..=MAP_HEIGHT { // horizontal lines
        let x_1: i32 = 0;
        let y_1: i32 = 50 * y * MAP_TILE_SIZE as i32;
        let x_2: i32 = 50 * MAP_WIDTH * MAP_TILE_SIZE as i32;
        let y_2: i32 = y_1;
        // world.renderer.render_line(x_1, y_1, x_2, y_2);
    }
}