use crate::World;
use crate::behavior::{behaviors, BehaviorState};
use crate::behavior::btree::{BehaviorTreeNode, Status};


pub trait Need {
    fn evaluate(&mut self);
    fn run_behavior(&mut self, state: &mut BehaviorState, world: &World) -> Status;
    fn get_value(&self) -> usize;
}

pub struct Eat {
    pub value: usize,
    pub behavior: Box<dyn BehaviorTreeNode>,
}

impl Need for Eat {
    fn evaluate(&mut self) {

    }

    fn run_behavior(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        self.behavior.run(state, world)
    }

    fn get_value(&self) -> usize {
        self.value
    }
}

impl Eat {
    pub fn new() -> Self {
        Self {
            value: 0,
            behavior: Box::new(behaviors::find_food()),
        }
    }
}