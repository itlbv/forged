use crate::map::Map;

pub fn sort_entities_by_proximity(_owner_entity: usize, mut _entities: &Vec<usize>) {}

pub fn find_free_tiles(start_tile_x: i32, start_tile_y: i32, w: i32, h: i32, margin: i32, map: &Map) -> (i32, i32) {
    let map_nodes = map.borrow_tiles();
    let node = map_nodes.borrow_node(5, 8);
    println!("{:?}", node);

    for node in map_nodes.iterator() {
    }
    (5, 7)
}