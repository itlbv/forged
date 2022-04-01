use crate::btree::Sequence;
use crate::build_tasks::{MakeBuildingTransparent, FindTilesAndPlaceInvisibleBuilding, FinishBuilding};
use crate::item_tasks::{ChooseIngredient, DropItemToMainTargetStorage, FindIngredients, PickUpTarget};
use crate::move_tasks::{MoveCloseToTarget, MoveToDestination};
use crate::tasks::{DoNothingTask, DoUntilFailure, EatTarget, FindFood, SetDestinationFromMainTarget, SetRecipe};
use crate::recipes;

pub fn find_food() -> Sequence {
    Sequence::of(vec![
        Box::new(FindFood::new()),
        Box::new(MoveCloseToTarget::new()),
        Box::new(EatTarget::new()),
    ])
}

pub fn build_house() -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipe::new( recipes::house())),
        Box::new(FindIngredients::new()),
        Box::new(FindTilesAndPlaceInvisibleBuilding::new()), //sets main target
        Box::new(MoveToDestination::new()),
        Box::new(MakeBuildingTransparent::new()),
        Box::new(collect_and_deliver_ingredients()),
        Box::new(FinishBuilding::new()),
    ])
}

pub fn collect_and_deliver_ingredients() -> DoUntilFailure {
    DoUntilFailure::of(vec![
        Box::new(ChooseIngredient::new()),
        Box::new(MoveCloseToTarget::new()),
        Box::new(PickUpTarget::new()),
        Box::new(SetDestinationFromMainTarget::new()),
        Box::new(MoveToDestination::new()),
        Box::new(DropItemToMainTargetStorage::new()),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}

pub fn test_path() -> MoveToDestination {
    MoveToDestination::new()
}

pub fn _test_building() -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipe::new(recipes::house())),
        Box::new(
            DoUntilFailure::of(vec![
                Box::new(FindTilesAndPlaceInvisibleBuilding::new())
            ])),
    ])
}