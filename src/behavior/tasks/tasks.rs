use crate::behavior::btree::{BehaviorTreeNode, Status};
use crate::behavior::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::components::{Food, Position, Recipe, Remove, Target, Destination, MainTarget, Building};
use crate::util::entity_util::mark_entity_for_removal;
use crate::util::{entity_util, map_util};
use crate::util::map_util::{pick_up_item_from_tile, place_item_to_tile};
use crate::util::physics::{distance_between, Vect};
use crate::{util, World};

pub struct SetDestinationFromMainTarget {}

impl BehaviorTreeNode for SetDestinationFromMainTarget {
    fn run(&mut self, owner: usize, world: &World) -> Status {
        self.set_destination(owner, world)
    }
}

impl SetDestinationFromMainTarget {
    pub fn new() -> Self { Self {} }

    fn set_destination(&self, owner: usize, world: &World) -> Status {
        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let main_target = main_targets.get(owner).unwrap().as_ref().unwrap();

        let buildings = world.ecs.borrow_component_vec::<Building>();
        let building = buildings.get(main_target.owner).unwrap().as_ref().unwrap();

        world.ecs.add_component_to_entity(owner, Destination::new(building.entry_x, building.entry_y));
        SUCCESS
    }
}

pub struct DoUntilFailure {
    pub children: Vec<Box<dyn BehaviorTreeNode>>,
    idx: i8,
}

impl DoUntilFailure {
    pub fn of(children: Vec<Box<dyn BehaviorTreeNode>>) -> Self {
        Self { children, idx: -1 }
    }
}

impl BehaviorTreeNode for DoUntilFailure {
    fn run(&mut self, owner: usize, world: &World) -> Status {
        loop {
            for (i, child) in self.children.iter_mut().enumerate() {
                if self.idx >= 0 && self.idx != i as i8 { continue; }

                let status = child.run(owner, world);
                if status == SUCCESS {
                    self.idx = -1;
                    continue;
                } else if status == RUNNING {
                    self.idx = i as i8;
                    return RUNNING;
                } else if status == FAILURE {
                    self.idx = -1;
                    return SUCCESS;
                }
            }
        }
    }
}

pub struct SetRecipe {
    recipe: Recipe,
}

impl BehaviorTreeNode for SetRecipe {
    fn run(&mut self, owner: usize, world: &World) -> Status {
        self.set_recipe(owner, world)
    }
}

impl SetRecipe {
    pub fn new(recipe: Recipe) -> Self {
        Self { recipe }
    }

    fn set_recipe(&self, owner: usize, world: &World) -> Status {
        world.ecs.add_component_to_entity(owner, self.recipe.clone());
        SUCCESS
    }
}

pub struct DoNothingTask {}

impl BehaviorTreeNode for DoNothingTask {
    fn run(&mut self, _: usize, _: &World) -> Status {
        SUCCESS
    }
}

impl DoNothingTask {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct FindFood {}

impl BehaviorTreeNode for FindFood {
    fn run(&mut self, owner: usize, world: &World) -> Status {
        self.find_food(owner, world)
    }
}

impl FindFood {
    pub fn new() -> Self {
        Self {}
    }

    fn find_food(&self, owner: usize, world: &World) -> Status {
        let foods = world.ecs.borrow_component_vec::<Food>();
        let positions = world.ecs.borrow_component_vec::<Position>();

        let own_pos_vect = Vect::of(
            positions.get(owner).unwrap().as_ref().unwrap().x,
            positions.get(owner).unwrap().as_ref().unwrap().y,
        );

        let zip = foods.iter().zip(positions.iter());
        let iter = zip.filter_map(
            |(food, pos)| Some((food.as_ref()?, pos.as_ref()?))
        );

        let mut target_entity_id: i32 = -1;
        let mut shortest_distance: f32 = 10000.0;
        for (_food, pos) in iter {
            let distance = distance_between(&own_pos_vect, &Vect::of(pos.x, pos.y));
            if distance < shortest_distance {
                shortest_distance = distance;
                target_entity_id = pos.entity_id as i32;
            }
        }

        if target_entity_id < 0 {
            return FAILURE;
        }

        world.ecs.add_component_to_entity(owner, Target::new(target_entity_id as usize));
        SUCCESS
    }
}

pub struct EatTarget {
}

impl BehaviorTreeNode for EatTarget {
    fn run(&mut self, owner: usize, world: &World) -> Status {
        self.eat(owner, world)
    }
}

impl EatTarget {
    pub fn new() -> Self {
        Self {  }
    }

    fn eat(&self, owner: usize, world: &World) -> Status {
        let targets = world.ecs.borrow_component_vec::<Target>();
        let target_id = targets.get(owner).unwrap().as_ref().unwrap().target_id;
        entity_util::mark_entity_for_removal(target_id, world);

        let positions = world.ecs.borrow_component_vec::<Position>();
        let target_pos = positions.get(target_id).unwrap().as_ref().unwrap();
        map_util::pick_up_item_from_tile(target_id, target_pos.x as usize, target_pos.y as usize, &world.map);

        SUCCESS
    }
}
