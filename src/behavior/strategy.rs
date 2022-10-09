use crate::behavior::behaviors::{build_house, do_nothing, find_food};
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::Knowledge;
use crate::util::log;
use crate::World;

pub fn calculate_behavior(knowledge: &Knowledge, _: &World) -> Box<dyn BehaviorTreeNode> {
    log::info("Calculating strategy", knowledge.owner);
    find_food()
}