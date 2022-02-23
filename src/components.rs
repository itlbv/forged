use crate::btree::BehaviorTreeNode;
use crate::World;

pub struct Behavior {
    pub behavior_tree: Box<dyn BehaviorTreeNode>,
}

impl Behavior {
    pub fn run(&self, world: &World) {
        self.behavior_tree.run(world);
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Name {
    pub v: String,
}

pub struct RenderShape {
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
