use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::{Name, Position, RenderShape, World};

pub enum Status {
    SUCCESS,
    FAILURE,
    RUNNING,
}

pub(crate) trait BehaviorTreeNode {
    fn run(&self, world: &World) -> Status;
}

pub struct Sequence {
    pub(crate) children: Vec<Box<dyn BehaviorTreeNode>>,
}

impl BehaviorTreeNode for Sequence {
    fn run(&self, world: &World) -> Status {
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