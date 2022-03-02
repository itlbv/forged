use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, SUCCESS};
use crate::components::{Inventory, Recipe, TargetEntity};
use crate::{util, World};

pub struct FindIngredients {
    owner_id: usize,
}

impl BehaviorTreeNode for FindIngredients {
    fn run(&self, world: &World) -> Status {
        self.find(world)
    }
}

impl FindIngredients {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn find(&self, world: &World) -> Status {
        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(self.owner_id).unwrap().as_ref().unwrap();

        let mut ingredients = Vec::new();
        for (ingr_type_id, amount) in &recipe.ingredients_type_ids {
            let items_of_type = world.ecs.get_entities_by_type_id(ingr_type_id);
            if items_of_type.len() < *amount { return FAILURE; }

            util::sort_entities_by_proximity(self.owner_id, &items_of_type);

            for i in 0..*amount {
                ingredients.push(items_of_type[i]);
            }
        }
        if ingredients.len() > 0 {
            util::sort_entities_by_proximity(self.owner_id, &ingredients);

            let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
            let inventory = inventories.get_mut(self.owner_id).unwrap().as_mut().unwrap();
            inventory.items_needed = ingredients;
            SUCCESS
        } else { FAILURE }
    }
}

pub struct ChooseIngredient {
    owner_id: usize,
}

impl BehaviorTreeNode for ChooseIngredient {
    fn run(&self, world: &World) -> Status {
        self.choose_ingredient(world)
    }
}

impl ChooseIngredient {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn choose_ingredient(&self, world: &World) -> Status {
        let inventories = world.ecs.borrow_component_vec::<Inventory>();
        let inventory = inventories.get(self.owner_id).unwrap().as_ref().unwrap();

        if inventory.items_needed.len() == 0 {
            println!("Items needed are empty!");
            return FAILURE;
        }

        world.ecs.add_component_to_entity(
            self.owner_id,
            TargetEntity::new(inventory.items_needed[0]));

        SUCCESS
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