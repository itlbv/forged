use std::collections::HashSet;
use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, SUCCESS};
use crate::components::{Inventory, Recipe};
use crate::World;

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

pub struct TargetIngredient {
    owner_id: usize,
}

impl BehaviorTreeNode for TargetIngredient {
    fn run(&self, world: &World) -> Status {
        self.target_ingredient(world)
    }
}

impl TargetIngredient {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn target_ingredient(&self, world: &World) -> Status {
        let inventories = world.ecs.borrow_component_vec::<Inventory>();
        let inventory = inventories.get(self.owner_id).unwrap().as_ref().unwrap();
        SUCCESS
    }
}