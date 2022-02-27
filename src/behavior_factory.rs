use crate::btree::Sequence;
use crate::tasks::{BuildHouseTask, DoNothingTask, EatTargetTask, FindFoodTask, FindPlaceToBuildTask, MoveCloseToTargetTask, MoveToPositionTask};

pub fn find_food_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFoodTask::new(owner_id)),
        Box::new(MoveCloseToTargetTask::new(owner_id)),
        Box::new(EatTargetTask::new(owner_id)),
    ])
}

pub fn build_house_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindPlaceToBuildTask::new(owner_id)),
        Box::new(MoveToPositionTask::new(owner_id)),
        Box::new(BuildHouseTask::new()),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}