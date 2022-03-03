use crate::btree::{BehaviorTreeNode, Status};
use crate::{entity_factory, World};
use crate::btree::Status::SUCCESS;
use crate::components::{TargetMain, TargetPosition};

pub struct FinishBuilding {
    own_id: usize,
}

impl BehaviorTreeNode for FinishBuilding {
    fn run(&mut self, world: &World) -> Status {
        self.finish_building(world)
    }
}

impl FinishBuilding {
    pub fn new(own_id: usize) -> Self {
        Self { own_id }
    }

    fn finish_building(&self, _world: &World) -> Status {
        SUCCESS
    }
}

pub struct BuildFoundation {
    owner_id: usize,
}

impl BehaviorTreeNode for BuildFoundation {
    fn run(&mut self, world: &World) -> Status {
        self.build(world)
    }
}

impl BuildFoundation {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn build(&self, world: &World) -> Status {
        let house_id = entity_factory::create_house(1.0, 1.0, world);
        world.ecs.add_component_to_entity(self.owner_id, TargetMain::new(house_id));
        SUCCESS
    }
}

pub struct FindPlaceToBuildTask {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindPlaceToBuildTask {
    fn run(&mut self, world: &World) -> Status {
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