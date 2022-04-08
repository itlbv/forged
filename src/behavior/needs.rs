use crate::World;
use crate::behavior::behaviors;
use crate::behavior::btree::{BehaviorTreeNode, Status};
use crate::components::BehaviorBlackboard;
use crate::ecs::EntityId;

pub trait Need {
    fn evaluate(&mut self);
    fn run_behavior(&mut self, blackboard: &mut BehaviorBlackboard, world: &World) -> Status;
    fn get_value(&self) -> usize;
}

pub struct Hunger {
    pub value: usize,
    pub behavior: Box<dyn BehaviorTreeNode>,
}

impl Need for Hunger {
    fn evaluate(&mut self) {

    }

    fn run_behavior(&mut self, blackboard: &mut BehaviorBlackboard, world: &World) -> Status {
        self.behavior.run(blackboard, world)
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