use crate::btree::Sequence;
use crate::build_tasks::{BuildHouseTask, FindPlaceToBuildTask};
use crate::item_tasks::CheckIfIngredientsAvailable;
use crate::move_tasks::{MoveCloseToTargetTask, MoveToPositionTask};
use crate::recipes;
use crate::tasks::{DoNothingTask, DoUntilFailure, EatTargetTask, FindFoodTask, SetRecipeTask};

pub fn find_food_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFoodTask::new(owner_id)),
        Box::new(MoveCloseToTargetTask::new(owner_id)),
        Box::new(EatTargetTask::new(owner_id)),
    ])
}

pub fn build_house_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(SetRecipeTask::new(owner_id, recipes::house())),
        Box::new(CheckIfIngredientsAvailable::new(owner_id)),
        Box::new(FindPlaceToBuildTask::new(owner_id)),
        Box::new(MoveToPositionTask::new(owner_id)),
        Box::new(BuildHouseTask::new()),
        Box::new(collect_ingredients_sequence(owner_id)),
        // finish building
    ])
}

pub fn collect_ingredients_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(
            DoUntilFailure::of(vec![
                // choose ingredient
                // move close to target
                // pick up target
            ])
        ),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}