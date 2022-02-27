use sdl2::libc::stat;
use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::World;

#[derive(PartialEq)] // needed for match expression
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
            let status = child.run(world);
            if status == SUCCESS {
                continue;
            } else { return status; }
            // match status {
            //     SUCCESS => continue,
            //     FAILURE => FAILURE,
            //     RUNNING => RUNNING,
            // };
        }
        SUCCESS
    }
}