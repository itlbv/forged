use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

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

    pub fn register_component<Comp: 'static>(&mut self) {
        self.component_registry.insert(TypeId::of::<Comp>(), Box::new(RefCell::new(vec![] as Vec<Option<Comp>>)));
    }

    pub fn create_entity(&self) -> usize {
        let new_entity_id = self.entity_count.take();
        self.entity_count.replace(new_entity_id + 1);
        for (_, comp) in &self.component_registry {
            comp.push_none();
        }
        new_entity_id
    }

    pub fn add_component_to_entity<Comp: 'static>(&self, entity_id: usize, comp: Comp) {
        self.borrow_component_vec_mut::<Comp>()[entity_id] = Some(comp);
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        for (_, comp_vec) in &self.component_registry {
            comp_vec.set_none_at_index(entity_id);
        }
    }

    pub fn get_entities_by_type_id(&self, type_id: &TypeId) -> Vec<usize> {
        self.component_registry
            .get(type_id)
            .unwrap()
            .collect_non_empty()
    }

    pub fn borrow_component_vec<Comp: 'static>(&self) -> Ref<'_, Vec<Option<Comp>>> {
        self.get_component_vec::<Comp>().borrow()
    }

    pub fn borrow_component_vec_mut<Comp: 'static>(&self) -> RefMut<'_, Vec<Option<Comp>>> {
        self.get_component_vec::<Comp>().borrow_mut()
    }

    fn get_component_vec<Comp: 'static>(&self) -> &RefCell<Vec<Option<Comp>>> {
        self.component_registry
            .get(&TypeId::of::<Comp>())
            .unwrap()
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<Comp>>>>()
            .unwrap()
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&self);
    fn set_none_at_index(&self, idx: usize);
    fn collect_non_empty(&self) -> Vec<usize>;
}

impl<Comp: 'static> ComponentVec for RefCell<Vec<Option<Comp>>> {
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

    fn collect_non_empty(&self) -> Vec<usize> {
        let mut entities = Vec::new();
        for (i, comp) in self.borrow().iter().enumerate() {
            if let Some(_) = comp.as_ref() {
                entities.push(i);
            }
        };
        entities
    }
}
