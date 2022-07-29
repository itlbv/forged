use crate::behavior::behaviors::do_nothing;
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::Knowledge;
use crate::util::log;
use crate::World;

pub fn calculate_behavior(knowledge: &Knowledge, _: &World) -> Box<dyn BehaviorTreeNode> {
    log::info("Calculating strategy", knowledge.owner);
    do_nothing()
}