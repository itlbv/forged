use crate::behavior::Brain;
use crate::World;

pub fn behavior(world: &World) {
    let mut brains = world.ecs.borrow_component_vec_mut::<Brain>();
    for brain in brains.iter_mut() {
        match brain {
            None => { continue; }
            Some(brain) => {
                if !brain.commands.is_empty() {
                    brain.commands.pop().unwrap().execute(brain);
                }

                if brain.behaviors.is_empty() {
                    panic!("Behavior {} has no actions!", brain.knowledge.owner);
                }
                brain.behaviors[0].run(&mut brain.knowledge, world);
            }
        };
    }
}
