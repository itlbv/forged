use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::World;

pub enum Status {
    SUCCESS,
    FAILURE,
    RUNNING,
}

pub trait BehaviorTreeNode {
    fn run(&self, world: &World) -> Status;
}

pub struct Sequence {
    pub children: Vec<Box<dyn BehaviorTreeNode>>,
}

impl Sequence {
    pub fn of(children: Vec<Box<dyn BehaviorTreeNode>>) -> Self {
        Self {
            children,
        }
    }
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