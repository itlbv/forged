use std::borrow::BorrowMut;
use crate::{entity_factory, main, util, World};
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
    pub own_id: usize,
}

impl BehaviorTreeNode for ClaimLand {
    fn run(&mut self, world: &World) -> Status {
        self.find_place(world)
    }
}

impl ClaimLand {
    pub fn new(own_id: usize) -> Self {
        Self { own_id }
    }

    fn find_place(&self, world: &World) -> Status {
        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(self.own_id).unwrap().as_ref().unwrap();
        let render_shape = &recipe.render_shape;

        let positions = world.ecs.borrow_component_vec::<Position>();
        let own_pos = positions.get(self.own_id).unwrap().as_ref().unwrap();

        let (x, y) = util::find_free_tiles(
            own_pos.x as i32,
            own_pos.y as i32,
            render_shape.w as i32,
            render_shape.h as i32,
            1,
            &world.map,
        );

        world.ecs.add_component_to_entity(self.own_id, Destination::new(x as f32 + 0.5, y as f32 + 0.5));
        SUCCESS
    }
}