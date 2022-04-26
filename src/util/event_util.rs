use std::borrow::BorrowMut;
use crate::behavior::{Behavior, OrderMove};
use crate::ecs::{Ecs, EntityId};
use crate::World;

pub fn dispatch_event(entity: EntityId, x: f32, y: f32, ecs: &Ecs) {
    let mut behaviors = ecs.borrow_component_vec_mut::<Behavior>();
    let mut behavior = behaviors.get_mut(entity).unwrap().as_mut();
    match behavior {
        None => {}
        Some(behavior) => {
            behavior.events.insert(0, Box::new(OrderMove::new(x, y)));
        }
    }
}