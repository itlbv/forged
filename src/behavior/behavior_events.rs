use crate::behavior::{Behavior, behaviors};
use crate::behavior::tasks::move_tasks::MoveToDestination;
use crate::util::physics::Vect;

pub trait BehaviorEvent {
    fn execute(&self, behavior: &mut Behavior);
}

pub struct OrderMove {
    pub x: f32,
    pub y: f32,
}

impl OrderMove {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl BehaviorEvent for OrderMove {
    fn execute(&self, behavior: &mut Behavior) {
        behavior.state.destination = Option::from(Vect::of(self.x, self.y));
        behavior.routine = Box::new(MoveToDestination::new())
    }
}