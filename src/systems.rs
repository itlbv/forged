use std::detect::__is_feature_detected::sha;
use crate::components::{Behavior, Position, Remove, RenderShape, Texture};
use crate::{Renderer, World};

use crate::constants::{MAP_HEIGHT, MAP_TILE_SIZE, MAP_WIDTH};
use crate::util_structs::Color;

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        let b = behavior.as_mut();
        match b {
            None => { continue; }
            Some(_) => {
                let behavior = b.unwrap();
                let needs = &mut behavior.needs;
                for need in needs.iter_mut() {
                    need.evaluate();
                }

                let mut priority_need_idx = 0;
                for i in 0..needs.len() {
                    if needs[i].get_value() > needs[priority_need_idx].get_value() {
                        priority_need_idx = i;
                    }
                }
                needs[priority_need_idx].run_behavior(behavior.owner, world);
            }
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

pub fn input(world: &mut World) {
    world.input_handler.update(&mut world.properties);
}

pub fn render(world: &mut World) {
    world.renderer.start_frame(&world.properties);
    render_map(world);
    render_textures(world);
    render_entities(world);
    world.renderer.present_frame();
}

pub fn render_textures(world: &mut World) {
    let textures = world.ecs.borrow_component_vec::<Texture>();
    let positions = world.ecs.borrow_component_vec::<Position>();

    let zip = textures.iter().zip(positions.iter());
    let iter = zip.filter_map(
        |(texture, pos)| Some((texture.as_ref()?, pos.as_ref()?))
    );

    for (texture_comp, pos) in iter {
        let texture_sprite = world.assets.borrow_texture(texture_comp.sprite_id.as_str());
        world.renderer.render_texture(
            texture_sprite,
            texture_comp.sprite_in_tileset_x,
            texture_comp.sprite_in_tileset_y,
            texture_comp.sprite_w,
            texture_comp.sprite_h,
            Renderer::world_to_screen(pos.x, world.properties.camera.zoom) - (texture_comp.render_offset_x * world.properties.camera.zoom as u32 / 50) as i32 + world.properties.camera.x,
            Renderer::world_to_screen(pos.y, world.properties.camera.zoom) - (texture_comp.render_offset_y * world.properties.camera.zoom as u32 / 50) as i32 + world.properties.camera.y,
            (texture_comp.object_w_tiles as f32 * world.properties.camera.zoom as f32 * texture_comp.scale) as u32,
            (texture_comp.object_h_tiles as f32 * world.properties.camera.zoom as f32 * texture_comp.scale) as u32,
        );
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
        world.renderer.render_rect(
            pos.x + shape.offset_x,
            pos.y + shape.offset_y,
            shape.w,
            shape.h,
            (shape.color.r, shape.color.g, shape.color.b, shape.color.a),
            &world.properties.camera,
        );

        // let x = Renderer::world_to_screen(pos.x + shape.offset_x, world.properties.zoom_factor);
        // let y = Renderer::world_to_screen(pos.y + shape.offset_y, world.properties.zoom_factor);
        // let w = Renderer::world_to_screen(shape.w, world.properties.zoom_factor);
        // let h = Renderer::world_to_screen(shape.h, world.properties.zoom_factor);
        // world.renderer.render_rect(x + world.properties.camera_x, y + world.properties.camera_y, w, h, shape.color.r, shape.color.g, shape.color.b, shape.color.a);
        // let _true_pos_x = Renderer::world_to_screen(pos.x, world.properties.zoom_factor);
        // let _true_pos_y = Renderer::world_to_screen(pos.y, world.properties.zoom_factor);
        // world.renderer.render_dot(true_pos_x + world.properties.camera_x, true_pos_y + world.properties.camera_y); // true position
    }
}

pub fn render_map(world: &mut World) {
    for tile in world.map.borrow_tiles().iterator() {
        let color: Color;
        if !tile.passable {
            color = Color::new(10, 10, 10, 255);
        } else {
            color = Color::new(tile.color.r, tile.color.g, tile.color.b, tile.color.a);
        };

        world.renderer.render_rect(
            tile.x as f32,
            tile.y as f32,
            MAP_TILE_SIZE,
            MAP_TILE_SIZE,
            (color.r, color.g, color.b, color.a),
            &world.properties.camera,
        );

        // world.renderer.render_texture(
        //     world.assets.borrow_texture("map_tileset"),
        //     tile.tileset_x, tile.tileset_y, tile.tileset_w, tile.tileset_h,
        //     x + world.properties.camera_x, y + world.properties.camera_y, world.properties.zoom_factor as u32, world.properties.zoom_factor as u32);
    }
    render_map_grid(world);
}

pub fn render_map_grid(world: &mut World) {
    for x in 0..=MAP_WIDTH { // vertical lines
        world.renderer.render_line(
            (x as f32 * MAP_TILE_SIZE, 0.),
            (x as f32 * MAP_TILE_SIZE, MAP_HEIGHT as f32 * MAP_TILE_SIZE),
            (0, 0, 0, 255),
            &world.properties.camera,
        );
    }

    for y in 0..=MAP_HEIGHT { // horizontal lines
        world.renderer.render_line(
            (0., y as f32 * MAP_TILE_SIZE),
            (MAP_WIDTH as f32 * MAP_TILE_SIZE, y as f32 * MAP_TILE_SIZE),
            (0, 0, 0, 255),
            &world.properties.camera,
        );
    }
}