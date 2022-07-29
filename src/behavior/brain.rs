use crate::behavior::behaviors;
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::commands::Command;
use crate::ecs::EntityId;
use crate::util::physics::Vect;

pub struct Brain {
    pub state: BehaviorState,
    pub commands: Vec<Box<dyn Command>>,
    pub behaviors: Vec<Box<dyn BehaviorTreeNode>>,
}

impl Brain {
    pub fn new(owner: EntityId) -> Self {
        Self {
            state: BehaviorState::new(owner),
            commands: vec![],
            behaviors: vec![Box::new(behaviors::do_nothing())]
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