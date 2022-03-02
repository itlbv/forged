use std::collections::HashSet;
use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::components::{Food, Inventory, Position, Recipe, Remove, TargetEntity, TargetPosition};
use crate::{entity_factory, items, World};
use crate::constants::MOB_SPEED;
use crate::physics::{distance_between, Vect, vector_to};

pub struct DoUntilFailure {
    pub children: Vec<Box<dyn BehaviorTreeNode>>,
}

impl DoUntilFailure {
    pub fn of(children: Vec<Box<dyn BehaviorTreeNode>>) -> Self {
        Self { children }
    }
}

impl BehaviorTreeNode for DoUntilFailure {
    fn run(&self, world: &World) -> Status {
        for child in &self.children {
            let status = child.run(world);
            if status == FAILURE {
                return FAILURE;
            }
        }
        RUNNING
    }
}

pub struct SetRecipeTask {
    owner_id: usize,
    recipe: Recipe,
}

impl BehaviorTreeNode for SetRecipeTask {
    fn run(&self, world: &World) -> Status {
        self.set_recipe(world)
    }
}

impl SetRecipeTask {
    pub fn new(owner_id: usize, recipe: Recipe) -> Self {
        Self { owner_id, recipe }
    }

    fn set_recipe(&self, world: &World) -> Status {
        world.ecs.add_component_to_entity(self.owner_id, self.recipe.clone());
        SUCCESS
    }
}

pub struct DoNothingTask {}

impl BehaviorTreeNode for DoNothingTask {
    fn run(&self, _: &World) -> Status {
        SUCCESS
    }
}

impl DoNothingTask {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct FindFoodTask {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindFoodTask {
    fn run(&self, world: &World) -> Status {
        self.find_food(world)
    }
}

impl FindFoodTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn find_food(&self, world: &World) -> Status {
        println!("find food");
        let foods = world.ecs.borrow_component_vec::<Food>();
        let positions = world.ecs.borrow_component_vec::<Position>();

        let own_pos_vect = Vect::of(
            positions.get(self.owner_id).unwrap().as_ref().unwrap().x,
            positions.get(self.owner_id).unwrap().as_ref().unwrap().y,
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
            println!("No food found");
            return FAILURE;
        }

        world.ecs.add_component_to_entity(self.owner_id, TargetEntity::new(target_entity_id as usize));
        SUCCESS
    }
}

pub struct EatTargetTask {
    owner_id: usize,
}

impl BehaviorTreeNode for EatTargetTask {
    fn run(&self, world: &World) -> Status {
        self.eat(world)
    }
}

impl EatTargetTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn eat(&self, world: &World) -> Status {
        println!("eat food");
        let targets = world.ecs.borrow_component_vec::<TargetEntity>();
        let target_id = targets.get(self.owner_id).unwrap().as_ref().unwrap().target_id;
        world.ecs.add_component_to_entity::<Remove>(target_id, Remove { owner_id: target_id });
        SUCCESS
    }
}
