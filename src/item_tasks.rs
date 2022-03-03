use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, SUCCESS};
use crate::components::{Inventory, Recipe, Remove, Storage, Target, MainTarget};
use crate::{util, World};

pub struct DropItemToMainTargetStorage {
    own_id: usize,
}

impl BehaviorTreeNode for DropItemToMainTargetStorage {
    fn run(&mut self, world: &World) -> Status {
        self.drop_item(world)
    }
}

impl DropItemToMainTargetStorage {
    pub fn new(owner_id: usize) -> Self { Self { own_id: owner_id } }

    fn drop_item(&self, world: &World) -> Status {
        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let main_target_id = main_targets.get(self.own_id).unwrap().as_ref().unwrap().own_id;

        let mut storages = world.ecs.borrow_component_vec_mut::<Storage>();
        let storage = storages.get_mut(main_target_id).unwrap().as_mut().unwrap();

        let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
        let mut own_inventory = inventories.get_mut(self.own_id).unwrap().as_mut().unwrap();

        storage.items.push(own_inventory.item_carried as usize);
        own_inventory.item_carried = -1;
        SUCCESS
    }
}

pub struct PickUpTarget {
    owner_id: usize,
}

impl BehaviorTreeNode for PickUpTarget {
    fn run(&mut self, world: &World) -> Status {
        self.pick_up(world)
    }
}

impl PickUpTarget {
    pub fn new(owner_id: usize) -> Self { Self { owner_id } }

    fn pick_up(&mut self, world: &World) -> Status {
        let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
        let mut inventory = inventories.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        let target_id = inventory.items_needed.remove(0);
        inventory.item_carried = target_id as i32;

        world.ecs.add_component_to_entity(target_id, Remove { owner_id: target_id });

        SUCCESS
    }
}

pub struct FindIngredients {
    owner_id: usize,
}

impl BehaviorTreeNode for FindIngredients {
    fn run(&mut self, world: &World) -> Status {
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
    fn run(&mut self, world: &World) -> Status {
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
            return FAILURE;
        }

        world.ecs.add_component_to_entity(
            self.owner_id,
            Target::new(inventory.items_needed[0]));

        SUCCESS
    }
}
