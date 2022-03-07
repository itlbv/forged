use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::map::Map;

pub fn sort_entities_by_proximity(_owner_entity: usize, mut _entities: &Vec<usize>) {}

pub fn find_free_tiles(start_tile_x: i32, start_tile_y: i32, w: i32, h: i32, margin: usize, map: &Map) -> (i32, i32) {
    println!("{}, {}", w, h);
    for i in 1..=5 {
        // node (-x, -y..y)
        // node (x, -y..y)
        for y in 0 - i..=i {
            if is_tile_suitable_for_building(i, y, w, h, margin, map) { return (i, y); }
            if is_tile_suitable_for_building(-i, y, w, h, margin, map) { return (-i, y); }
        }
        // node (-x..x, -y)
        // node (-x..x, y)
        for x in 0 - i..=i {
            if is_tile_suitable_for_building(x, i, w, h, margin, map) { return (x, i); }
            if is_tile_suitable_for_building(x, -i, w, h, margin, map) { return (x, -i); }
        }
    }

    (-1, -1)
}

fn is_tile_suitable_for_building(start_tile_x: i32, start_tile_y: i32, w: i32, h: i32, margin: usize, map: &Map) -> bool {
    if start_tile_x < 0 || start_tile_y < 0 || start_tile_x >= MAP_WIDTH || start_tile_y >= MAP_HEIGHT {
        return false;
    }

    let map_tiles = map.borrow_tiles();
    println!("check start tile: {}, {}", start_tile_x, start_tile_y);

    for y in 0..h {
        for x in 0..w {
            let new_x = start_tile_x + x;
            let new_y = start_tile_y + y;

            if new_x < 0 || new_y < 0 || new_x >= MAP_WIDTH || new_y >= MAP_HEIGHT {
                continue;
            }

            println!("check: {}, {}", new_x, new_y);
            let tile = map_tiles.borrow_tile(new_x, new_y);
            if !tile.walkable || tile.occupied {
                println!("not suitable: {}, {}", new_x, new_y);
                return false;
            }
        }
    }

    println!("start tile suitable: {}, {}", start_tile_x, start_tile_y);
    true
}