use crate::components::Position;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::ecs::{Ecs, EntityId};
use crate::map::Map;
use crate::util::physics::distance_between;

pub fn entity_closest_to_pos(x: f32, y: f32, entities: Vec<EntityId>, ecs: &Ecs) -> Option<EntityId> {
    let mut closest_entity = None;
    let mut min_distance = 0.5;
    for entity in entities {
        let entity_pos = entity_position(entity, ecs);
        let distance_to_entity = distance_between(x, y, entity_pos.0, entity_pos.1);
        if distance_to_entity < min_distance {
            closest_entity = Some(entity);
            min_distance = distance_to_entity;
        }
    }
    closest_entity
}

pub fn entity_position(entity: EntityId, ecs: &Ecs) -> (f32, f32) {
    let positions = ecs.borrow_component_vec::<Position>();
    let pos = positions.get(entity).unwrap().as_ref().unwrap();
    (pos.x , pos.y)
}

pub fn sort_entities_by_proximity(_owner_entity: usize, mut _entities: &Vec<usize>) {}

pub fn claim_tiles(start_tile_x: usize, start_tile_y: usize, w: usize, h: usize, map: &Map) {
    let mut map_tiles = map.borrow_tiles_mut();
    for y in 0..h {
        for x in 0..w {
            let mut tile = map_tiles.borrow_tile_mut(start_tile_x + x, start_tile_y + y);
            tile.passable = false;
        }
    }
}

pub fn find_free_tiles(start_x: usize, start_y: usize, w: usize, h: usize, margin: usize, map: &Map) -> Option<(usize, usize)> {
    for i in 1..=50 {
        // node (-x, -y..y)
        // node (x, -y..y)
        for y in 0 - i..=i {
            if start_x + i < MAP_WIDTH - 1 {
                if is_tile_suitable_for_building(start_x + i, y, w, h, margin, map) { return Some((start_x + i, y)); }
            }
            if i <= start_x {
                if is_tile_suitable_for_building(start_x - i, y, w, h, margin, map) { return Some((start_x - i, y)); }
            }
        }
        // node (-x..x, -y)
        // node (-x..x, y)
        for x in 0 - i..=i {
            if start_y + i < MAP_HEIGHT - 1 {
                if is_tile_suitable_for_building(x, start_y + i, w, h, margin, map) { return Some((x, start_y + i)); }
            }
            if i <= start_y {
                if is_tile_suitable_for_building(x, start_y - i, w, h, margin, map) { return Some((x, start_y - i)); }
            }
        }
    }
    None
}

fn is_tile_suitable_for_building(start_tile_x: usize, start_tile_y: usize, w: usize, h: usize, margin: usize, map: &Map) -> bool {
    if margin > start_tile_x
        || margin > start_tile_y {
        return false;
    }

    let start_tile_with_margin_x = start_tile_x - margin;
    let start_tile_with_margin_y = start_tile_y - margin;
    let w_with_margin = w + margin * 2;
    let h_with_margin = h + margin * 2;

    if start_tile_with_margin_x + w_with_margin > MAP_WIDTH
        || start_tile_with_margin_y + h_with_margin > MAP_HEIGHT {
        return false;
    }

    let map_tiles = map.borrow_tiles();

    for y in 0..h_with_margin {
        for x in 0..w_with_margin {
            let new_x = start_tile_with_margin_x + x;
            let new_y = start_tile_with_margin_y + y;

            if new_x >= MAP_WIDTH || new_y >= MAP_HEIGHT {
                continue;
            }

            let tile = map_tiles.borrow_tile(new_x, new_y);
            if !tile.passable {
                return false;
            }
        }
    }
    true
}

pub fn put_entity_to_tile(item: EntityId, x: usize, y: usize, map: &Map) {
    map.set_tile_passable(x, y, false);
    map.put_entity_to_tile(item, x, y);
}

pub fn pick_up_item_from_tile(item: usize, x: usize, y: usize, map: &Map) {
    map.set_tile_passable(x, y, true);
    map.remove_entity_from_tile(item, x, y)
}
