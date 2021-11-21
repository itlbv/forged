struct NameComponent(&'static str);
struct HealthComponent(i32);

struct World {
    entity_count: usize,
    name_comps: Vec<Option<NameComponent>>,
    health_comps: Vec<Option<HealthComponent>>,
}

impl World {
    fn new() -> Self {
        Self {
            entity_count: 0,
            name_comps: Vec::new(),
            health_comps: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        self.name_comps.push(None);
        self.health_comps.push(None);
        self.entity_count += 1;
        self.entity_count
    }

    fn add_components_to_entity(&mut self, name_comp: NameComponent, health_comp: HealthComponent, entity: usize) {
        self.name_comps[entity - 1] = Some(name_comp);
        self.health_comps[entity - 1] = Some(health_comp);
    }
}

fn main() {
    let mut world = World::new();
    let new_entity_id = world.new_entity();
    world.add_components_to_entity(NameComponent("Alice"), HealthComponent(5), new_entity_id);
}
