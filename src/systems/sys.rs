use crate::{World};

use crate::components::{Label, Remove};
use crate::util::text_util;

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
                        &mut world.assets,
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
