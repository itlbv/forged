use crate::{behaviors, World};
use crate::btree::{BehaviorTreeNode, Status};


pub trait Need {
    fn evaluate(&mut self);
    fn run_behavior(&mut self, owner:usize, world: &World) -> Status;
    fn get_value(&self) -> usize;
}

pub struct Hunger {
    pub value: usize,
    pub behavior: Box<dyn BehaviorTreeNode>,
}

impl Need for Hunger {
    fn evaluate(&mut self) {

    }

    fn run_behavior(&mut self, owner:usize, world: &World) -> Status {
        self.behavior.run(owner, world)
    }

    fn get_value(&self) -> usize {
        self.value
    }
}

impl Hunger {
    pub fn new() -> Self {
        Self {
            value: 0,
            behavior: Box::new(behaviors::find_food()),
        }
    }
}