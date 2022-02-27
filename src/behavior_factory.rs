use crate::btree::Sequence;
use crate::tasks::{DoNothingTask, EatTargetTask, FindFoodTask, MoveCloseToTargetTask};

pub fn find_food_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFoodTask::new(owner_id)),
        Box::new(MoveCloseToTargetTask::new(owner_id)),
        Box::new(EatTargetTask::new(owner_id)),
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask {}
}