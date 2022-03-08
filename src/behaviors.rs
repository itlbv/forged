use crate::btree::Sequence;
use crate::build_tasks::{MakeBuildingTransparent, FindTilesAndPlaceInvisibleBuilding, FinishBuilding};
use crate::item_tasks::{ChooseIngredient, DropItemToMainTargetStorage, FindIngredients, PickUpTarget};
use crate::move_tasks::{MoveCloseToTarget, MoveToDestination};
use crate::tasks::{DoNothingTask, DoUntilFailure, EatTarget, FindFood, SetDestinationFromMainTarget, SetRecipe};
use crate::recipes;

pub fn find_food(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFood::new(owner_id)),
        Box::new(MoveCloseToTarget::new(owner_id)),
        Box::new(EatTarget::new(owner_id)),
    ])
}

pub fn test_building(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipe::new(owner_id, recipes::house())),
        Box::new(
            DoUntilFailure::of(vec![
                Box::new(FindTilesAndPlaceInvisibleBuilding::new(owner_id))
            ])),
    ])
}

pub fn build_house(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipe::new(owner_id, recipes::house())),
        Box::new(FindIngredients::new(owner_id)),
        Box::new(FindTilesAndPlaceInvisibleBuilding::new(owner_id)), //sets main target
        Box::new(MoveToDestination::new(owner_id)),
        Box::new(MakeBuildingTransparent::new(owner_id)),
        Box::new(deliver_ingredients(owner_id)),
        // make building usable
        // Box::new(FinishBuilding::new(owner_id)),
    ])
}

pub fn deliver_ingredients(owner_id: usize) -> DoUntilFailure {
    DoUntilFailure::of(vec![
        Box::new(ChooseIngredient::new(owner_id)),
        Box::new(MoveCloseToTarget::new(owner_id)),
        Box::new(PickUpTarget::new(owner_id)),
        Box::new(SetDestinationFromMainTarget::new(owner_id)),
        Box::new(MoveToDestination::new(owner_id)),
        Box::new(DropItemToMainTargetStorage::new(owner_id)),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}
