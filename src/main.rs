mod ecs;
use crate::ecs::*;

fn main() {
    let mut world = World::new();
    let alice_entity_id = world.new_entity();
    world.add_components_to_entity(NameComponent("Alice"), HealthComponent(5), alice_entity_id);
    let bob_entity_id = world.new_entity();
    world.add_components_to_entity(NameComponent("Bob"), HealthComponent(9), bob_entity_id);

    let mut counter = 0;
    while counter < 10 {
        hunger_system(&mut world);
        counter += 1;
    }
}
