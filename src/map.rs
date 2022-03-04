use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;

pub struct Map {
    pub nodes: Vec<MapNode>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            nodes: Map::create_map(),
        }
    }

    fn create_map() -> Vec<MapNode> {
        let mut map_nodes = vec![];
        for y in 0..MAP_HEIGHT {
            for x in 00..MAP_WIDTH {
                map_nodes.push(MapNode::new(x, y, true, Color::new(85, 125, 70, 255)));
            }
        }
        map_nodes
    }
}

pub struct MapNode {
    pub x: i32,
    pub y: i32,
    pub passable: bool,
    pub occupied: bool,
    pub color: Color,
}

impl MapNode {
    fn new(x: i32, y: i32, passable: bool, color: Color) -> Self {
        Self {
            x,
            y,
            passable,
            occupied: false,
            color }
    }
}
