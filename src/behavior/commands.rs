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
        behavior.state.destination = Some(Vect::of(self.x, self.y));
        behavior.behaviors.insert(0, behaviors::move_to_destination());
    }
}

impl MoveToDestinationCommand {
    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self { x, y })
    }
}