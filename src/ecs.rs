use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};

pub struct Ecs {
    entity_count: RefCell<usize>,
    component_registry: HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            entity_count: RefCell::new(0),
            component_registry: Default::default(),
        }
    }

    pub fn register_component<C: 'static>(&mut self) {
        self.component_registry.insert(TypeId::of::<C>(), Box::new(RefCell::new(vec![] as Vec<Option<C>>)));
    }

    pub fn create_entity(&self) -> usize {
        let new_entity_id = self.entity_count.take();
        self.entity_count.replace(new_entity_id + 1);
        for (_, comp) in &self.component_registry {
            comp.push_none();
        }
        new_entity_id
    }

    pub fn add_component_to_entity<C: 'static>(&self, entity_id: usize, comp: C) {
        self.borrow_component_vec_mut::<C>()[entity_id] = Some(comp);
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        for (_, comp_vec) in &self.component_registry {
            comp_vec.set_none_at_index(entity_id);
        }
    }

    pub fn get_entities_by_type_id(&self, type_id: &TypeId) -> HashSet<usize> {
        self.component_registry
            .get(type_id)
            .unwrap()
            .collect_non_empty()
    }

    pub fn borrow_component_vec<C: 'static>(&self) -> Ref<'_, Vec<Option<C>>> {
        self.get_component_vec::<C>().borrow()
    }

    pub fn borrow_component_vec_mut<C: 'static>(&self) -> RefMut<'_, Vec<Option<C>>> {
        self.get_component_vec::<C>().borrow_mut()
    }

    fn get_component_vec<T: 'static>(&self) -> &RefCell<Vec<Option<T>>> {
        self.component_registry
            .get(&TypeId::of::<T>())
            .unwrap()
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<T>>>>()
            .unwrap()
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&self);
    fn set_none_at_index(&self, idx: usize);
    fn collect_non_empty(&self) -> HashSet<usize>;
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&self) {
        match self.try_borrow_mut() {
            Ok(mut vec) => { vec.push(None); }
            Err(_) => {
                unsafe {
                    let vec = &mut *self.as_ptr();
                    vec.push(None);
                }
            }
        }
    }

    fn set_none_at_index(&self, idx: usize) {
        self.borrow_mut()[idx] = None;
    }

    fn collect_non_empty(&self) -> HashSet<usize> {
        let mut entities = HashSet::new();
        for (i, comp) in self.borrow().iter().enumerate() {
            if let Some(c) = comp.as_ref() {
                entities.insert(i);
            }
        };
        entities
    }
}
