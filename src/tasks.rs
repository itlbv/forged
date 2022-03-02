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

pub struct CheckIfIngredientsAvailable {
    owner_id: usize,
}

impl BehaviorTreeNode for CheckIfIngredientsAvailable {
    fn run(&self, world: &World) -> Status {
        self.check(world)
    }
}

impl CheckIfIngredientsAvailable {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn check(&self, world: &World) -> Status {
        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(self.owner_id).unwrap().as_ref().unwrap();
        let mut items = HashSet::new();
        for (item_type_id, amount) in &recipe.ingredients_type_ids {
            let items_of_type = world.ecs.get_entities_by_type_id(item_type_id);
            if items_of_type.len() >= *amount {
                items.extend(items_of_type);
            } else {
                items.clear();
                break;
            }
        }
        if items.len() > 0 {
            let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
            let inventory = inventories.get_mut(self.owner_id).unwrap().as_mut().unwrap();
            inventory.items_needed = items;
            SUCCESS
        } else {
            FAILURE
        }
    }
}

pub struct CollectIngredients {
    owner_id: usize,
}

impl BehaviorTreeNode for CollectIngredients {
    fn run(&self, world: &World) -> Status {
        self.collect(world)
    }
}

impl CollectIngredients {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn collect(&self, world: &World) -> Status {
        SUCCESS
        // let inventories = world.ecs.borrow_component_vec::<Inventory>();
        // let inventory = inventories.get(self.owner_id).unwrap().as_ref().unwrap();
        // let mut items = HashSet::new();
        // for (item_type_id, amount) in &recipe.ingredients_type_ids {
        //     let items_of_type = world.ecs.get_entities_by_type_id(item_type_id);
        //     if items_of_type.len() >= *amount {
        //         items.extend(items_of_type);
        //     } else {
        //         items.clear();
        //         break;
        //     }
        // }
        // if items.len() > 0 {
        //     let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
        //     let inventory = inventories.get_mut(self.owner_id).unwrap().as_mut().unwrap();
        //     inventory.items_needed = items;
        //     SUCCESS
        // } else {
        //     FAILURE
        // }
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

pub struct MoveToPositionTask {
    owner_id: usize,
}

impl BehaviorTreeNode for MoveToPositionTask {
    fn run(&self, world: &World) -> Status {
        self.move_to(world)
    }
}

impl MoveToPositionTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn move_to(&self, world: &World) -> Status {
        let target_positions = world.ecs.borrow_component_vec::<TargetPosition>();
        let target_pos = target_positions.get(self.owner_id).unwrap().as_ref().unwrap();

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let own_pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if distance_between(&Vect::of(target_pos.x, target_pos.y), &Vect::of(own_pos.x, own_pos.y)) < 0.02 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &Vect::of(target_pos.x, target_pos.y),
            world.delta_time);

        own_pos.x += velocity_vec.x;
        own_pos.y += velocity_vec.y;
        RUNNING
    }
}

pub struct MoveCloseToTargetTask {
    owner_id: usize,
}

impl BehaviorTreeNode for MoveCloseToTargetTask {
    fn run(&self, world: &World) -> Status {
        self.move_close(world)
    }
}

impl MoveCloseToTargetTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn move_close(&self, world: &World) -> Status {
        println!("move close to");
        let targets = world.ecs.borrow_component_vec::<TargetEntity>();
        let target_id = targets.get(self.owner_id).unwrap().as_ref().unwrap().target_id;

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();

        let destination = Vect::of(
            positions.get(target_id).unwrap().as_ref().unwrap().x,
            positions.get(target_id).unwrap().as_ref().unwrap().y,
        );

        let own_pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if distance_between(&destination, &Vect::of(own_pos.x, own_pos.y)) < 0.5 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &destination,
            world.delta_time);

        own_pos.x += velocity_vec.x;
        own_pos.y += velocity_vec.y;
        RUNNING
    }
}

fn get_velocity_vec_to(start_pos: &Vect, destination: &Vect, delta_time: f32) -> Vect {
    let mut velocity = vector_to(start_pos, destination);
    velocity.normalize();
    velocity.set_length(velocity.length() * delta_time * MOB_SPEED);
    velocity
}