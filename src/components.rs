use std::any::TypeId;
use std::collections::HashMap;
use crate::btree::BehaviorTreeNode;
use crate::btree::Status::SUCCESS;
use crate::{behaviors, World};

pub struct Behavior {
    pub behavior_tree: Box<dyn BehaviorTreeNode>,
}

impl Behavior {
    pub fn run(&mut self, world: &World) {
        if self.behavior_tree.run(world) == SUCCESS {
            self.behavior_tree = Box::new(behaviors::do_nothing());
        }
    }
}

pub struct Inventory {
    pub item_carried: i32,
    pub items_needed: Vec<usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            item_carried: -1,
            items_needed: Vec::new(),
        }
    }
}

pub struct Storage {
    pub items: Vec<usize>
}

impl Storage {
    pub fn new() -> Self {
        Self { items: Vec::new(), }
    }
}

pub struct Recipe {
    pub ingredients_type_ids: HashMap<TypeId, usize>,
    pub render_shape: RenderShape,
}

impl Clone for Recipe {
    fn clone(&self) -> Self {
        Self {
            ingredients_type_ids: self.ingredients_type_ids.clone(),
            render_shape: self.render_shape.clone(),
        }
    }
}

pub struct Destination {
    pub x: f32,
    pub y: f32,
}

impl Destination {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Target {
    pub target_id: usize,
}

impl Target {
    pub fn new(target_id: usize) -> Self {
        Self { target_id }
    }
}

pub struct MainTarget {
    pub own_id: usize,
}

impl MainTarget {
    pub fn new(target_id: usize) -> Self {
        Self { own_id: target_id }
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub entity_id: usize,
}

impl Position {
    pub fn of(x: f32, y: f32, entity_id: usize) -> Self {
        Self { x, y, entity_id }
    }
}

pub struct Name {
    pub v: String,
}

pub struct RenderShape {
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Clone for RenderShape {
    fn clone(&self) -> Self {
        Self {
            w: self.w,
            h: self.h,
            color: Color {
                r: self.color.r,
                g: self.color.g,
                b: self.color.b
            }
        }
    }
}

pub struct Food {}

pub struct Remove {
    pub owner_id: usize,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
