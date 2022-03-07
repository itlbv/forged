use crate::map::Map;

pub fn sort_entities_by_proximity(_owner_entity: usize, mut _entities: &Vec<usize>) {}

pub fn find_free_tiles(start_tile_x: i32, start_tile_y: i32, w: i32, h: i32, margin: i32, map: &Map) -> (i32, i32) {
    let map_nodes = map.borrow_tiles();
    let node = map_nodes.borrow_node(5, 8);
    println!("{:?}", node);
    for node in map_nodes.iterator() {}

    for i in 1..=5 {
        // node (-x, -y..y)
        // node (x, -y..y)
        for y in 0 - i..=i {
            println!("{}, {}", i, y);
            println!("{}, {}", -i, y);
        }
        // node (-x..x, -y)
        // node (-x..x, y)
        for x in 0 - i..=i {
            println!("{}, {}", x, i);
            println!("{}, {}", x, -i);
        }
    }

    (5, 7)
}