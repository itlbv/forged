use crate::btree::Sequence;
use crate::build_tasks::{BuildFoundation, FindPlaceToBuildTask, FinishBuilding};
use crate::item_tasks::{ChooseIngredient, DropItemToMainTargetStorage, FindIngredients, PickUpTarget};
use crate::move_tasks::{MoveCloseToTargetEntity, MoveToPositionTask};
use crate::recipes;
use crate::tasks::{DoNothingTask, DoUntilFailure, EatTargetTask, FindFoodTask, SetDestinationFromMainTarget, SetRecipeTask};

pub fn find_food_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFoodTask::new(owner_id)),
        Box::new(MoveCloseToTargetEntity::new(owner_id)),
        Box::new(EatTargetTask::new(owner_id)),
    ])
}

pub fn build_house(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipeTask::new(owner_id, recipes::house())),
        Box::new(FindIngredients::new(owner_id)),
        Box::new(FindPlaceToBuildTask::new(owner_id)),
        Box::new(MoveToPositionTask::new(owner_id)),
        Box::new(BuildFoundation::new(owner_id)), //sets main target
        Box::new(deliver_ingredients(owner_id)),
        Box::new(FinishBuilding::new(owner_id)),
    ])
}

pub fn deliver_ingredients(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(
            DoUntilFailure::of(vec![
                Box::new(ChooseIngredient::new(owner_id)),
                Box::new(MoveCloseToTargetEntity::new(owner_id)),
                Box::new(PickUpTarget::new(owner_id)),
                Box::new(SetDestinationFromMainTarget::new(owner_id)),
                Box::new(MoveToPositionTask::new(owner_id)),
                Box::new(DropItemToMainTargetStorage::new(owner_id)),
            ])
        ),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}