use crate::btree::Sequence;
use crate::tasks::{DoNothingTask, FindFoodTask};

pub fn find_food_sequence(owner_id: usize) -> Sequence {
    Sequence::of(vec![
        Box::new(FindFoodTask { owner_id })
    ])
}

pub fn do_nothing() -> DoNothingTask {
    DoNothingTask{}
}