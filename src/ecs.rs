pub struct NameComponent(pub &'static str);

pub struct HealthComponent(pub i32);
impl HealthComponent {
    fn decrease(&mut self) {
        self.0 -= 1;
    }
}

pub struct World {
    entity_count: usize,
    name_comps: Vec<Option<NameComponent>>,
    health_comps: Vec<Option<HealthComponent>>,
}
impl World {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            name_comps: Vec::new(),
            health_comps: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        self.name_comps.push(None);
        self.health_comps.push(None);
        self.entity_count += 1;
        self.entity_count
    }

    pub fn add_components_to_entity(
        &mut self,
        name_comp: NameComponent,
        health_comp: HealthComponent,
        entity: usize,
    ) {
        self.name_comps[entity - 1] = Some(name_comp);
        self.health_comps[entity - 1] = Some(health_comp);
    }
}

pub fn hunger_system(world: &mut World) {
    let comps = world
        .health_comps
        .iter_mut()
        .zip(world.name_comps.iter())
        .filter_map(
            |(health, name): (&mut Option<HealthComponent>, &Option<NameComponent>)| {
                Some((health.as_mut()?, name.as_ref()?))
            },
        );
    for (health, name) in comps {
        health.decrease();
        if health.0 <= 0 {
            println!("{} died of hunger!", name.0);
        } else {
            println!("{} is getting hungry!", name.0);
        }
    }
}

