use crate::behavior::btree::Sequence;
use crate::behavior::tasks::build_tasks::{MakeBuildingTransparent, FindTilesAndPlaceInvisibleBuilding, FinishBuilding};
use crate::behavior::tasks::item_tasks::{ChooseIngredient, DropItemToMainTargetStorage, FindIngredients, PickUpTarget};
use crate::behavior::tasks::move_tasks::{MoveCloseToTarget, MoveToDestination};
use crate::behavior::tasks::tasks::{DoNothingTask, DoUntilFailure, EatTarget, FindNearestFood, SetDestinationFromMainTarget, SetRecipe};
use crate::recipes;

pub fn move_to_destination() -> Box<MoveToDestination> {
    MoveToDestination::boxed()
}

pub fn find_food() -> Sequence {
    Sequence::of(vec![
        Box::new(FindNearestFood::new()),
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

pub fn _test_building() -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipe::new(recipes::house())),
        Box::new(
            DoUntilFailure::of(vec![
                Box::new(FindTilesAndPlaceInvisibleBuilding::new())
            ])),
    ])
}