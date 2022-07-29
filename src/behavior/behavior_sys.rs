use crate::util::log;
use crate::behavior::{behaviors, Brain};
use crate::World;

pub fn behavior(world: &World) {
    let mut brains = world.ecs.borrow_component_vec_mut::<Brain>();
    for brain in brains.iter_mut() {
        match brain {
            None => { continue; }
            Some(brain) => {
                if !brain.commands.is_empty() {
                    brain.commands.pop().unwrap().execute(brain, world);
                }

                if brain.behaviors.is_empty() {
                    log::error("No assigned behavior! Assigning DoNothing.", brain.knowledge.owner);
                    brain.behaviors.push(behaviors::do_nothing());
                }
                brain.behaviors[0].run(&mut brain.knowledge, world);
            }
        };
    }
}
