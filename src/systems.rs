use crate::components::{Behavior, Position, Remove, RenderShape};
use crate::{Renderer, World};
use crate::constants::{MAP_HEIGHT, MAP_NODE_SIZE, MAP_WIDTH};

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        let b = behavior.as_mut();
        match b {
            None => { continue; }
            Some(_) => { b.unwrap().run(world) }
        };
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
        let x = Renderer::world_to_screen(pos.x);
        let y = Renderer::world_to_screen(pos.y);
        let w = Renderer::world_to_screen(shape.w);
        let h = Renderer::world_to_screen(shape.h);
        world.renderer.render_rect(x - w / 2, y - h / 2, w, h, shape.color.r, shape.color.g, shape.color.b, shape.color.a);
        world.renderer.render_dot(x, y); //render true position
    }
}

pub fn render_map(world: &mut World) {
    for map_node in world.map.borrow_tiles().iterator() {
        let x = Renderer::world_to_screen(map_node.x as f32);
        let y = Renderer::world_to_screen(map_node.y as f32);
        let node_size = Renderer::world_to_screen(MAP_NODE_SIZE);
        world.renderer.render_rect(
            x,
            y,
            node_size,
            node_size,
            map_node.color.r, map_node.color.g, map_node.color.b, map_node.color.a);
        world.renderer.render_dot(x, y); // true position
    }

    for x in 0..=MAP_WIDTH { // vertical lines
        let x_1: i32 = 50 * x * MAP_NODE_SIZE as i32;
        let y_1: i32 = 0;
        let x_2: i32 = x_1;
        let y_2: i32 = 50 * MAP_HEIGHT * MAP_NODE_SIZE as i32;
        world.renderer.render_line(x_1, y_1, x_2, y_2);
    }

    for y in 0..=MAP_HEIGHT { // horizontal lines
        let x_1: i32 = 0;
        let y_1: i32 = 50 * y * MAP_NODE_SIZE as i32;
        let x_2: i32 = 50 * MAP_WIDTH * MAP_NODE_SIZE as i32;
        let y_2: i32 = y_1;
        world.renderer.render_line(x_1, y_1, x_2, y_2);
    }
}