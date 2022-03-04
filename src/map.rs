use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;

pub struct MapNode {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl MapNode {
    fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }
}

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
                map_nodes.push(MapNode { x, y, color: Color { r: 85, g: 125, b: 70 } });
            }
        }
        map_nodes
    }
}