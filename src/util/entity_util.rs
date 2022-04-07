use crate::components::Remove;
use crate::World;

pub fn mark_entity_for_removal(entity_id: usize, world: &World) {
    world.ecs.add_component_to_entity::<Remove>(entity_id, Remove::new(entity_id));
}