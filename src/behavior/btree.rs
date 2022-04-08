use crate::behavior::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::components::BehaviorState;
use crate::World;

#[derive(PartialEq)] // needed for match expression
pub enum Status {
    SUCCESS,
    FAILURE,
    RUNNING,
}

pub trait BehaviorTreeNode {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status;
}

pub struct Sequence {
    pub children: Vec<Box<dyn BehaviorTreeNode>>,
    idx: i8,
}

impl Sequence {
    pub fn of(children: Vec<Box<dyn BehaviorTreeNode>>) -> Self {
        Self {
            children,
            idx: -1,
        }
    }
}

impl BehaviorTreeNode for Sequence {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        for (i, child) in self.children.iter_mut().enumerate() {
            if self.idx >= 0 && self.idx != i as i8 { continue; }

            let status = child.run(state, world);
            if status == SUCCESS {
                self.idx = -1;
                continue;
            } else if status == RUNNING {
                self.idx = i as i8;
                return RUNNING;
            } else if status == FAILURE {
                self.idx = -1;
                return FAILURE;
            }
        }
        SUCCESS
    }
}