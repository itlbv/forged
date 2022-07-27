use crate::{Behavior, World};
use crate::behavior::behaviors;
use crate::util::physics::Vect;

pub trait Command {
    fn execute(&self, behavior: &mut Behavior) {}
}

pub struct MoveToDestinationCommand {
    x: f32,
    y: f32,
}

impl Command for MoveToDestinationCommand {
    fn execute(&self, behavior: &mut Behavior) {
        behavior.behaviors.insert(0, behaviors::move_to_spot(self.x, self.y));
    }
}

impl MoveToDestinationCommand {
    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self { x, y })
    }
}