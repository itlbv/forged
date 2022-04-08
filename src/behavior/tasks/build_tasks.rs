use crate::{entities, World};
use crate::behavior::btree::{BehaviorTreeNode, Status};
use crate::behavior::btree::Status::{FAILURE, SUCCESS};
use crate::components::{BehaviorBlackboard, Building, Destination, MainTarget, Position, Recipe, RenderShape};
use crate::ecs::EntityId;
use crate::util::{log, map_util};

pub struct FinishBuilding {}

impl BehaviorTreeNode for FinishBuilding {
    fn run(&mut self, blackboard: &mut BehaviorBlackboard, world: &World) -> Status {
        self.finish_building(blackboard.owner, world)
    }
}

impl FinishBuilding {
    pub fn new() -> Self {
        Self {}
    }

    fn finish_building(&self, owner: EntityId, world: &World) -> Status {
        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let own_main_target = main_targets.get(owner).unwrap().as_ref().unwrap();
        let building_id = own_main_target.owner;

        let mut buildings = world.ecs.borrow_component_vec_mut::<Building>();
        let mut building = buildings.get_mut(building_id).unwrap().as_mut().unwrap();
        building.usable = true;

        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(owner).unwrap().as_ref().unwrap();

        world.ecs.add_component_to_entity(building_id, RenderShape::from_recipe(recipe));
        SUCCESS
    }
}

pub struct MakeBuildingTransparent {}

impl BehaviorTreeNode for MakeBuildingTransparent {
    fn run(&mut self, blackboard: &mut BehaviorBlackboard, world: &World) -> Status {
        self.make_transparent(blackboard.owner, world)
    }
}

impl MakeBuildingTransparent {
    pub fn new() -> Self {
        Self {}
    }

    fn make_transparent(&self, owner: usize, world: &World) -> Status {
        let mut recipe;
        {
            let recipes = world.ecs.borrow_component_vec::<Recipe>();
            recipe = recipes.get(owner).unwrap().as_ref().unwrap().clone();
        }

        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let main_target = main_targets.get(owner).unwrap().as_ref().unwrap();

        recipe.render_shape.set_transparent();
        world.ecs.add_component_to_entity(main_target.owner, recipe.render_shape);
        SUCCESS
    }
}

pub struct FindTilesAndPlaceInvisibleBuilding {}

impl BehaviorTreeNode for FindTilesAndPlaceInvisibleBuilding {
    fn run(&mut self, blackboard: &mut BehaviorBlackboard, world: &World) -> Status {
        self.find_tiles(blackboard.owner, world)
    }
}

impl FindTilesAndPlaceInvisibleBuilding {
    pub fn new() -> Self {
        Self {}
    }

    fn find_tiles(&self, owner: usize, world: &World) -> Status {
        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(owner).unwrap().as_ref().unwrap();
        let render_shape = &recipe.render_shape;

        let own_x: f32;
        let own_y: f32;
        {
            let positions = world.ecs.borrow_component_vec::<Position>();
            let own_pos = positions.get(owner).unwrap().as_ref().unwrap();
            own_x = own_pos.x;
            own_y = own_pos.y;
        }

        let build_pos = map_util::find_free_tiles(
            own_x as usize,
            own_y as usize,
            render_shape.w as usize,
            render_shape.h as usize,
            1,
            &world.map,
        );

        if build_pos.is_none() {
            log::warn("Can't find place to build.", owner);
            return FAILURE;
        }

        let (x, y) = build_pos.unwrap();
        map_util::claim_tiles(x, y, render_shape.w as usize, render_shape.h as usize, &world.map);
        let (house_id, house_entry_x, house_entry_y) = entities::house_from_recipe(x as f32, y as f32, recipe.clone(), world);

        world.ecs.add_component_to_entity(owner, Destination::new(house_entry_x, house_entry_y));
        world.ecs.add_component_to_entity(owner, MainTarget::new(house_id));
        SUCCESS
    }
}