use crate::{entity_factory, World};
use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::SUCCESS;
use crate::components::{MainTarget, Destination, Recipe};

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
    own_id: usize,
}

impl BehaviorTreeNode for BuildFoundation {
    fn run(&mut self, world: &World) -> Status {
        self.build(world)
    }
}

impl BuildFoundation {
    pub fn new(own_id: usize) -> Self {
        Self { own_id }
    }

    fn build(&self, world: &World) -> Status {
        let recipe;
        {
            let recipes = world.ecs.borrow_component_vec::<Recipe>();
            recipe = recipes.get(self.own_id).unwrap().as_ref().unwrap().clone();
        }
        let foundation_id = entity_factory::foundation(5.5, 6.5, recipe, world);

        world.ecs.add_component_to_entity(self.own_id, MainTarget::new(foundation_id));
        SUCCESS
    }
}

pub struct FindPlaceToBuild {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindPlaceToBuild {
    fn run(&mut self, world: &World) -> Status {
        self.find_place(world)
    }
}

impl FindPlaceToBuild {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn find_place(&self, world: &World) -> Status {
        world.ecs.add_component_to_entity(self.owner_id, Destination::new(5.5, 7.5));
        SUCCESS
    }
}