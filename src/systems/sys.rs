use crate::{World};
use crate::behavior::Behavior;
use crate::components::{Label, Remove};
use crate::util::text_util;

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
                needs[priority_need_idx].run_behavior(&mut behavior.state, world);
            }
        };
    }
}

pub fn update_labels_textures(world: &mut World) {
    let mut labels = world.ecs.borrow_component_vec_mut::<Label>();
    for label in labels.iter_mut() {
        match label {
            None => { continue; }
            Some(label) => {
                if label.updated {
                    text_util::update_text_in_asset_manager(
                        &label.label_id,
                        label.borrow_text(),
                        &mut world.assets
                    );
                    label.updated = false;
                }
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
