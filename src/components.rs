use std::collections::HashMap;
use crate::btree::BehaviorTreeNode;
use crate::{behavior_factory, World};
use crate::btree::Status::SUCCESS;

pub struct Behavior {
    pub behavior_tree: Box<dyn BehaviorTreeNode>,
}

impl Behavior {
    pub fn run(&mut self, world: &World) {
        if self.behavior_tree.run(world) == SUCCESS {
            self.behavior_tree = Box::new(behavior_factory::do_nothing());
        }
    }
}

pub struct Item {}

pub struct TargetPosition {
    pub x: f32,
    pub y: f32,
}

impl TargetPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct TargetEntity {
    pub target_id: usize,
}

impl TargetEntity {
    pub fn new(target_id: usize) -> Self {
        Self { target_id }
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

pub struct Food {}

pub struct Remove {
    pub owner_id: usize,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
