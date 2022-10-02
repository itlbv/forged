use crate::{Brain, World};
use crate::behavior::{behaviors, strategy};
use crate::util::log;


pub trait Command {
    fn execute(&self, _brain: &mut Brain, _world: &World) {}
}

pub struct MoveToSpotCommand {
    x: f32,
    y: f32,
}

impl Command for MoveToSpotCommand {
    fn execute(&self, brain: &mut Brain, _: &World) {
        log::info("Executing MoveToSpot command.", brain.knowledge.owner);
        brain.behaviors.insert(0, behaviors::move_to_spot(self.x, self.y));
    }
}

impl MoveToSpotCommand {
    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self { x, y })
    }
}

pub struct CalculateStrategy {}

impl Command for CalculateStrategy {
    fn execute(&self, brain: &mut Brain, world: &World) {
        log::info("Executing CalculateStrategy command.", brain.knowledge.owner);
        brain.behaviors.insert(0, strategy::calculate_behavior(&brain.knowledge, world));
    }
}

impl CalculateStrategy {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}