use crate::Brain;
use crate::behavior::behaviors;
use crate::util::physics::Vect;

pub trait Command {
    fn execute(&self, brain: &mut Brain) {}
}

pub struct MoveToDestinationCommand {
    x: f32,
    y: f32,
}

impl Command for MoveToDestinationCommand {
    fn execute(&self, brain: &mut Brain) {
        brain.behaviors.insert(0, behaviors::move_to_spot(self.x, self.y));
    }
}

impl MoveToDestinationCommand {
    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self { x, y })
    }
}

pub struct CalculateStrategy {}

impl Command for CalculateStrategy {
    fn execute(&self, brain: &mut Brain) {
        todo!()
    }
}

impl CalculateStrategy {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}