use crate::{Behavior, MAP_HEIGHT, MAP_WIDTH, Position, Renderer, RenderShape, World};
use crate::map::MAP_NODE_SIZE;

pub fn behavior_sys(world: &mut World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        let b = behavior.as_ref();
        // b.unwrap().run(world);
    }
}

pub fn render_entities_sys(world: &mut World) {
    let mut shapes = world.ecs.borrow_component_vec_mut::<RenderShape>();
    let mut positions = world.ecs.borrow_component_vec_mut::<Position>();

    let zip = shapes.iter_mut().zip(positions.iter_mut());
    let iter = zip.filter_map(
        |(shape, pos)| Some((shape.as_ref()?, pos.as_ref()?))
    );

    for (shape, pos) in iter {
        let x = Renderer::world_to_screen(pos.x);
        let y = Renderer::world_to_screen(pos.y);
        let w = Renderer::world_to_screen(shape.w);
        let h = Renderer::world_to_screen(shape.h);
        world.renderer.render_rect(x - w / 2, y - h / 2, w, h, shape.color.r, shape.color.g, shape.color.b);
        world.renderer.render_dot(x, y); //render true position
    }
}

pub fn render_map_sys(world: &mut World) {
    for map_node in &world.map.nodes {
        let x = Renderer::world_to_screen(map_node.x as f32);
        let y = Renderer::world_to_screen(map_node.y as f32);
        let node_size = Renderer::world_to_screen(MAP_NODE_SIZE);
        world.renderer.render_rect(
            x,
            y,
            node_size,
            node_size,
            map_node.color.r, map_node.color.g, map_node.color.b);
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