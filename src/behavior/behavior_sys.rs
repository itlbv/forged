use crate::behavior::Behavior;
use crate::World;

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        match behavior {
            None => { continue; }
            Some(behavior) => {
                if behavior.actions.is_empty() {
                    panic!("Behavior {} has no actions!", behavior.state.owner);
                }
                behavior.actions[0].run(&mut behavior.state, world);
            }
        };
    }
}
