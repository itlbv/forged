use crate::behavior::btree::{BehaviorTreeNode, Status};
use crate::behavior::btree::Status::{FAILURE, SUCCESS};
use crate::components::{BehaviorState, Inventory, MainTarget, Position, Recipe, Storage, Target};
use crate::util::entity_util::mark_entity_for_removal;
use crate::World;
use crate::util::{entity_util, map_util};

pub struct DropItemToMainTargetStorage {}

impl BehaviorTreeNode for DropItemToMainTargetStorage {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        self.drop_item(state.owner, world)
    }
}

impl DropItemToMainTargetStorage {
    pub fn new() -> Self { Self {} }

    fn drop_item(&self, owner: usize, world: &World) -> Status {
        let main_targets = world.ecs.borrow_component_vec::<MainTarget>();
        let main_target_id = main_targets.get(owner).unwrap().as_ref().unwrap().owner;

        let mut storages = world.ecs.borrow_component_vec_mut::<Storage>();
        let storage = storages.get_mut(main_target_id).unwrap().as_mut().unwrap();

        let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
        let mut own_inventory = inventories.get_mut(owner).unwrap().as_mut().unwrap();

        storage.items.push(own_inventory.item_carried as usize);
        own_inventory.item_carried = -1;
        SUCCESS
    }
}

pub struct PickUpTarget {}

impl BehaviorTreeNode for PickUpTarget {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        self.pick_up(state.owner, world)
    }
}

impl PickUpTarget {
    pub fn new() -> Self { Self {} }

    fn pick_up(&mut self, owner: usize, world: &World) -> Status {
        let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
        let mut inventory = inventories.get_mut(owner).unwrap().as_mut().unwrap();

        let target_id = inventory.items_needed.remove(0);
        inventory.item_carried = target_id as i32;

        entity_util::mark_entity_for_removal(target_id, world);

        let positions = world.ecs.borrow_component_vec::<Position>();
        let item_position = positions.get(target_id).unwrap().as_ref().unwrap();
        map_util::pick_up_item_from_tile(target_id, item_position.x as usize, item_position.y as usize, &world.map);

        SUCCESS
    }
}

pub struct FindIngredients {}

impl BehaviorTreeNode for FindIngredients {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        self.find(state.owner, world)
    }
}

impl FindIngredients {
    pub fn new() -> Self {
        Self {}
    }

    fn find(&self, owner: usize, world: &World) -> Status {
        let recipes = world.ecs.borrow_component_vec::<Recipe>();
        let recipe = recipes.get(owner).unwrap().as_ref().unwrap();

        let mut ingredients = Vec::new();
        for (ingr_type_id, amount) in &recipe.ingredients_type_ids {
            let items_of_type = world.ecs.get_entities_with_component_type_id(ingr_type_id);
            if items_of_type.len() < *amount { return FAILURE; }

            map_util::sort_entities_by_proximity(owner, &items_of_type);

            for i in 0..*amount {
                ingredients.push(items_of_type[i]);
            }
        }
        if ingredients.len() > 0 {
            map_util::sort_entities_by_proximity(owner, &ingredients);

            let mut inventories = world.ecs.borrow_component_vec_mut::<Inventory>();
            let inventory = inventories.get_mut(owner).unwrap().as_mut().unwrap();
            inventory.items_needed = ingredients;
            SUCCESS
        } else { FAILURE }
    }
}

pub struct ChooseIngredient {}

impl BehaviorTreeNode for ChooseIngredient {
    fn run(&mut self, state: &mut BehaviorState, world: &World) -> Status {
        self.choose_ingredient(state.owner, world)
    }
}

impl ChooseIngredient {
    pub fn new() -> Self {
        Self {}
    }

    fn choose_ingredient(&self, owner: usize, world: &World) -> Status {
        let inventories = world.ecs.borrow_component_vec::<Inventory>();
        let inventory = inventories.get(owner).unwrap().as_ref().unwrap();

        if inventory.items_needed.len() == 0 {
            return FAILURE;
        }

        world.ecs.add_component_to_entity(
            owner,
            Target::new(inventory.items_needed[0]));

        SUCCESS
    }
}
