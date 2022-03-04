use std::cell::{Ref, RefCell, RefMut};
use std::slice::Iter;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;

pub struct Map {
    nodes: Box<RefCell<dyn MapNodesVec>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            nodes: Map::create_map(),
        }
    }

    fn create_map() -> Box<RefCell<dyn MapNodesVec>> {
        let map_nodes = RefCell::new(vec![]);
        for y in 0..MAP_HEIGHT {
            for x in 00..MAP_WIDTH {
                map_nodes.borrow_mut().push(MapNode::new(x, y, true, Color::new(85, 125, 70, 255)));
            }
        }
        Box::new(map_nodes)
    }

    pub fn set_node_walkability(&self, x: i32, y: i32, walkable: bool) {
        let node = self.borrow_nodes().borrow_node(0, 0);
    }

    // fn borrow_node(&self, x: i32, y: i32) -> &MapNode {
    //     let nodes = self.borrow_nodes();
    //     nodes.borrow_node(x, y)
    // }

    pub fn borrow_nodes(&self) -> Ref<dyn MapNodesVec> {
        self.nodes.borrow()
    }

    pub fn borrow_nodes_mut(&self) -> RefMut<dyn MapNodesVec> {
        self.nodes.borrow_mut()
    }
}

pub trait MapNodesVec {
    fn borrow_node(&self, x: i32, y: i32) -> &MapNode;
    fn borrow_node_mut(&mut self, x: i32, y: i32) -> &mut MapNode;
    fn iterator(&self) -> Iter<MapNode>;
}

impl MapNodesVec for Vec<MapNode> {
    fn borrow_node(&self, x: i32, y: i32) -> &MapNode {
        let value = self.get(0);
        value.unwrap()
    }

    fn borrow_node_mut(&mut self, x: i32, y: i32) -> &mut MapNode {
        let value = self.get_mut(0);
        value.unwrap()
    }

    fn iterator(&self) -> Iter<MapNode> {
        let i = self.iter();
        return self.iter();
    }
}

#[derive(Debug)]
pub struct MapNode {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub occupied: bool,
    pub color: Color,
}

impl MapNode {
    fn new(x: i32, y: i32, walkable: bool, color: Color) -> Self {
        Self {
            x,
            y,
            walkable,
            occupied: false,
            color,
        }
    }
}
