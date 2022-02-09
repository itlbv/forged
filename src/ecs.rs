use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;

type EntityIdx = usize;

pub trait Component {}

struct ComponentVec {
    vec: Vec<Option<Box<dyn Component>>>,
}

pub struct Ecs {
    entity_count: EntityIdx,
    comps_registry: HashMap<TypeId, ComponentVec>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            comps_registry: Default::default(),
        }
    }

    pub fn register_component<C: Component + 'static>(&mut self) {
        self.comps_registry.insert(TypeId::of::<C>(), ComponentVec{vec: vec![]});
    }

    pub fn create_entity(&mut self) -> EntityIdx {
        self.entity_count += 1;
        for (_, comp_vec) in self.comps_registry.borrow_mut() {
            comp_vec.vec.push(Option::None);
        }
        self.entity_count
    }

    pub fn add_component_to_entity<C: Component + 'static>(&mut self, entity_idx: EntityIdx, comp: C) {
        self.comps_registry
            .get_mut(&comp.type_id())
            .unwrap()
            .vec
            .insert(entity_idx, Option::Some(Box::new(comp)));
    }
}