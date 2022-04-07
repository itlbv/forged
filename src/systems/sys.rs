use crate::components::{Behavior, Remove};
use crate::{World};

pub fn behavior(world: &World) {
    let mut behaviors = world.ecs.borrow_component_vec_mut::<Behavior>();
    for behavior in behaviors.iter_mut() {
        let b = behavior.as_mut();
        match b {
            None => { continue; }
            Some(_) => {
                let behavior = b.unwrap();
                let needs = &mut behavior.needs;
                for need in needs.iter_mut() {
                    need.evaluate();
                }

                let mut priority_need_idx = 0;
                for i in 0..needs.len() {
                    if needs[i].get_value() > needs[priority_need_idx].get_value() {
                        priority_need_idx = i;
                    }
                }
                needs[priority_need_idx].run_behavior(behavior.owner, world);
            }
        };
    }
}

pub fn remove_entities(world: &mut World) {
    let mut entity_ids_to_remove: Vec<usize> = vec![];
    {
        let removals = world.ecs.borrow_component_vec::<Remove>();
        let iter = removals.iter().filter_map(|removal| Some(removal.as_ref()?)).clone();
        for removal in iter {
            entity_ids_to_remove.push(removal.own_id);
        }
    }
    for entity_id in entity_ids_to_remove {
        world.ecs.remove_entity(entity_id);
    }
}

pub fn input(world: &mut World) {
    world.input_handler.update(&mut world.properties);
}
