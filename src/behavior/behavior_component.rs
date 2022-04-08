use crate::behavior::behaviors;
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::needs::{Hunger, Need};
use crate::ecs::EntityId;
use crate::util::physics::Vect;

pub struct Behavior {
    pub needs: Vec<Box<dyn Need>>,
    pub behavior_tree: Box<dyn BehaviorTreeNode>,
    pub state: BehaviorState,
}

impl Behavior {
    pub fn new(owner: EntityId) -> Self {
        Self {
            needs: vec![
                Box::new(Hunger::new()),
            ],
            behavior_tree: Box::new(behaviors::do_nothing()),
            state: BehaviorState::new(owner),
        }
    }
}

pub struct BehaviorState {
    pub owner: EntityId,
    pub target: Option<EntityId>,
    pub main_target: Option<EntityId>,
    pub destination: Option<Vect>,
}

impl BehaviorState {
    pub fn new(owner: EntityId) -> Self {
        Self {
            owner,
            target: None,
            main_target: None,
            destination: None,
        }
    }
}