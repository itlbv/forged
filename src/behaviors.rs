use crate::btree::Sequence;
use crate::build_tasks::{BuildHouseFoundation, FindPlaceToBuildTask};
use crate::item_tasks::{ChooseIngredient, FindIngredients, PickUpTarget};
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
        Box::new(FindIngredients::new(owner_id)),
        Box::new(FindPlaceToBuildTask::new(owner_id)),
        Box::new(MoveToPositionTask::new(owner_id)),
        Box::new(BuildHouseFoundation::new()),
        Box::new(collect_ingredients_sequence(owner_id)),
        // finish building
    ])
}

pub fn collect_ingredients_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(
            DoUntilFailure::of(vec![
                Box::new(ChooseIngredient::new(owner_id)),
                Box::new(MoveCloseToTargetTask::new(owner_id)),
                Box::new(PickUpTarget::new(owner_id)),
            ])
        ),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}