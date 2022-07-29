use crate::behavior::{Brain, behaviors, commands};
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::commands::MoveToSpotCommand;
use crate::ecs::{Action, Ecs, EntityId};
use crate::util::physics::Vect;


pub struct OrderEntityMoveToDestinationAction {
    entity: EntityId,
    command: Option<Box<MoveToSpotCommand>>,
    // x: f32,
    // y: f32,
}

impl Action for OrderEntityMoveToDestinationAction {
    fn execute(&mut self, ecs: &Ecs) {
        let mut behaviors = ecs.borrow_component_vec_mut::<Brain>();
        let behavior_component = behaviors.get_mut(self.entity).unwrap().as_mut().unwrap();
        if let Some(command) = self.command.take() {
            // behavior_component.state.destination = Some(Vect::of(self.x, self.y));
            behavior_component.commands.push(command);
        }
    }
}

impl OrderEntityMoveToDestinationAction {
    pub fn boxed(entity: EntityId, x: f32, y: f32) -> Box<Self> {
        Box::new(Self {
            command: Some(commands::move_to_spot(x, y)),
            entity,
            // x, y,
        })
    }
}