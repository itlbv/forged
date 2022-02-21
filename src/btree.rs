use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::World;

pub enum Status {
    SUCCESS,
    FAILURE,
    RUNNING,
}

pub(crate) trait BehaviorTreeNode {
    fn run(&self, world: &mut World) -> Status;
}

pub struct MoveTask {
    owner_id: usize,
    dest_x: f32,
    dest_y: f32,
}

impl MoveTask {
    pub fn new(owner_id: usize, x: f32, y: f32) -> Self {
        Self { owner_id, dest_x: x, dest_y: y }
    }

    fn move_to(&self, world: &mut World) -> Status {
        SUCCESS
    }
}

impl BehaviorTreeNode for MoveTask {
    fn run(&self, world: &mut World) -> Status {
        self.move_to(world)
    }
}

pub struct Sequence {
    pub(crate) children: Vec<Box<dyn BehaviorTreeNode>>,
}

impl BehaviorTreeNode for Sequence {
    fn run(&self, world: &mut World) -> Status {
        for child in &self.children {
            match child.run(world) {
                SUCCESS => { continue; }
                FAILURE => { FAILURE }
                RUNNING => { RUNNING }
            };
        }
        SUCCESS
    }
}