use crate::{entity_factory, main, World};
use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::SUCCESS;
use crate::components::{MainTarget, Destination, Recipe, Remove, Position};

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

    fn finish_building(&self, world: &World) -> Status {
        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let own_main_target = main_targets.get(self.own_id).unwrap().as_ref().unwrap();
        let foundation_id = own_main_target.own_id;

        let foundation_x;
        let foundation_y;
        {
            let positions = world.ecs.borrow_component_vec::<Position>();
            let foundation_pos = positions.get(foundation_id).unwrap().as_ref().unwrap();
            foundation_x = foundation_pos.x;
            foundation_y = foundation_pos.y;
        }

        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(foundation_id).unwrap().as_ref().unwrap();

        // create house entity
        entity_factory::house(foundation_x, foundation_y, recipe.clone(), world);

        // remove foundation entity
        world.ecs.add_component_to_entity(foundation_id, Remove::new(foundation_id));
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
        let foundation_id = entity_factory::foundation(5.5, 5.5, recipe, world);

        world.ecs.add_component_to_entity(self.own_id, MainTarget::new(foundation_id));
        SUCCESS
    }
}

pub struct ClaimLand {
    pub owner_id: usize,
}

impl BehaviorTreeNode for ClaimLand {
    fn run(&mut self, world: &World) -> Status {
        self.find_place(world)
    }
}

impl ClaimLand {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn find_place(&self, world: &World) -> Status {
        let mut map_nodes = world.map.borrow_nodes_mut();
        let map_node = map_nodes.borrow_node_mut(0, 0);
        map_node.walkable = false;

        world.ecs.add_component_to_entity(self.owner_id, Destination::new(5.5, 7.5));
        SUCCESS
    }
}