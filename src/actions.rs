use crate::behavior::{Behavior, behaviors};
use crate::behavior::btree::BehaviorTreeNode;
use crate::ecs::{Action, Ecs, EntityId};
use crate::util::physics::Vect;


pub struct MoveToDestinationAction {
    behavior: Option<Box<dyn BehaviorTreeNode>>,
    x: f32,
    y: f32,
}

impl Action for MoveToDestinationAction {
    fn execute(&mut self, target_entity: EntityId, ecs: &Ecs) {
        let mut behaviors = ecs.borrow_component_vec_mut::<Behavior>();
        let behavior_component = behaviors.get_mut(target_entity).unwrap().as_mut().unwrap();
        if let Some(behavior) = self.behavior.take() {
            behavior_component.state.destination = Some(Vect::of(self.x, self.y));
            behavior_component.behaviors.insert(0, behavior);
        }
    }
}

impl MoveToDestinationAction {
    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self {
            behavior: Some(behaviors::move_to_destination()),
            x,
            y,
        })
    }
}