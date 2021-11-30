pub struct MapNode {
    pub x: i32,
    pub y: i32,
}

pub struct Map {
    pub nodes: Vec<MapNode>,
}

impl Map {
    pub(crate) fn new() -> Self {
        Self{
            nodes: vec![
                MapNode{ x: 1, y: 1 },
                MapNode{ x: 3, y: 1 },
                MapNode{ x: 1, y: 3 },
                MapNode{ x: 3, y: 3 },
            ]
        }
    }
}