use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Add;
use std::process::id;
use std::slice::SliceIndex;

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

    pub fn add_component_to_entity_mut<C: 'static>(&mut self, entity_id: usize, comp: C) {
        self.component_registry
            .get_mut(&TypeId::of::<C>())
            .unwrap()
            .as_any_mut()
            .downcast_mut::<RefCell<Vec<Option<C>>>>()
            .unwrap()
            .get_mut()
            [entity_id] = Some(comp);
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        for (_, comp_vec) in &self.component_registry {
            comp_vec.set_none_at_index(entity_id);
        }
    }

    pub fn borrow_component_vec_mut<C: 'static>(&self) -> RefMut<'_, Vec<Option<C>>> {
        self.component_registry
            .get(&TypeId::of::<C>())
            .unwrap()
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<C>>>>()
            .unwrap()
            .borrow_mut()
    }

    pub fn borrow_component_vec<C: 'static>(&self) -> Ref<'_, Vec<Option<C>>> {
        self.component_registry
            .get(&TypeId::of::<C>())
            .unwrap()
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<C>>>>()
            .unwrap()
            .borrow()
    }

    pub fn borrow_component_vec_mut_option<C: 'static>(&self) -> Option<RefMut<Vec<Option<C>>>> {
        let vec = self.component_registry.get(&TypeId::of::<C>()).unwrap();
        if let Some(v) = vec
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<C>>>>() {
            return Some(v.borrow_mut());
        }
        None
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&self);
    fn set_none_at_index(&self, idx: usize);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&self) {
        self.borrow_mut().push(None);
    }

    fn set_none_at_index(&self, idx: usize) {
        self.borrow_mut()[idx] = None;
    }
}
