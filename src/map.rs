use crate::Color;

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
        for y in 0..25 {
            for x in 00..25 {
                map_nodes.push(MapNode { x, y, color: Color { r: 85, g: 125, b: 70 } });
            }
        }
        map_nodes
    }
}