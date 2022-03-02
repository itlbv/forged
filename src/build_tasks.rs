use crate::btree::{BehaviorTreeNode, Status};
use crate::{entity_factory, World};
use crate::btree::Status::SUCCESS;
use crate::components::TargetPosition;

pub struct BuildHouseTask {}

impl BehaviorTreeNode for BuildHouseTask {
    fn run(&self, world: &World) -> Status {
        self.build(world)
    }
}

impl BuildHouseTask {
    pub fn new() -> Self {
        Self {}
    }

    fn build(&self, world: &World) -> Status {
        entity_factory::create_house(1.0, 1.0, world);
        SUCCESS
    }
}

pub struct FindPlaceToBuildTask {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindPlaceToBuildTask {
    fn run(&self, world: &World) -> Status {
        self.find_place(world)
    }
}

impl FindPlaceToBuildTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn find_place(&self, world: &World) -> Status {
        world.ecs.add_component_to_entity(self.owner_id, TargetPosition::new(5.5, 7.5));
        SUCCESS
    }
}