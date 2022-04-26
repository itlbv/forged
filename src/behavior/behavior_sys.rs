use crate::behavior::Behavior;
use crate::World;

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        match behavior {
            None => { continue; }
            Some(behavior) => {
                behavior.routine.run(&mut behavior.state, world);
            }
        };
    }
}

pub fn react_to_events(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        match behavior {
            None => { continue; }
            Some(behavior) => {
                if !behavior.events.is_empty() {
                    behavior.events.pop().unwrap().execute(behavior);
                }
            }
        };
    }
}

// let needs = &mut behavior.needs;
// for need in needs.iter_mut() {
//     need.evaluate();
// }
//
// let mut priority_need_idx = 0;
// for i in 0..needs.len() {
//     if needs[i].get_value() > needs[priority_need_idx].get_value() {
//         priority_need_idx = i;
//     }
// }
// needs[priority_need_idx].run_behavior(&mut behavior.state, world);