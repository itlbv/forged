use std::borrow::BorrowMut;
use crate::util::log;
use crate::behavior::{behaviors, Brain};
use crate::behavior::btree::Status;
use crate::World;

pub fn behavior(world: &World) {
    let mut brains = world.ecs.borrow_component_vec_mut::<Brain>();
    for brain in brains.iter_mut() {
        if let Some(brain) = brain {
            if let Some(command) = brain.commands.pop() {
                command.execute(brain, world);
            }

            if let Some(behavior) = brain.behaviors.get_mut(0) {
                match behavior.run(&mut brain.knowledge, world) {
                    Status::SUCCESS | Status::FAILURE => { brain.behaviors.remove(0); }
                    _ => {}
                }
            } else {
                log::warn("No assigned behavior! Assigning DoNothing.", brain.knowledge.owner);
                brain.behaviors.push(behaviors::do_nothing());
            }
        }
    }
}
