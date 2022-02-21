use crate::Color;

pub const MAP_WIDTH: i32 = 15;
pub const MAP_HEIGHT: i32 = 10;
pub const MAP_NODE_SIZE: f32 = 1.0;

pub struct MapNode {
    pub x: i32,
    pub y: i32,
    pub(crate) color: Color,
}

pub struct Map {
    pub nodes: Vec<MapNode>,
}

impl Map {
    pub(crate) fn new() -> Self {
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