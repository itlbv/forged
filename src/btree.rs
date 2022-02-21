use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::{Name, Position, RenderShape, World};

pub enum Status {
    SUCCESS,
    FAILURE,
    RUNNING,
}

pub(crate) trait BehaviorTreeNode {
    fn run(&self, world: &World) -> Status;
}

pub struct MoveTask {
    owner_id: usize,
    dest_x: f32,
    dest_y: f32,
}

impl MoveTask {
    pub fn new(owner_id: usize, x: f32, y: f32) -> Self {
        Self { owner_id, dest_x: x, dest_y: y }
    }

    fn move_to(&self, world: &World) -> Status {
        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let position = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();
        if (self.dest_x - position.x) < 0.1 {
            return SUCCESS;
        }
        position.x += 0.01;
        position.y += 0.01;
        RUNNING
    }
}

impl BehaviorTreeNode for MoveTask {
    fn run(&self, world: &World) -> Status {
        self.move_to(world)
    }
}

pub struct Sequence {
    pub(crate) children: Vec<Box<dyn BehaviorTreeNode>>,
}

impl BehaviorTreeNode for Sequence {
    fn run(&self, world: &World) -> Status {
        for child in &self.children {
            match child.run(world) {
                SUCCESS => { continue; }
                FAILURE => { FAILURE }
                RUNNING => { RUNNING }
            };
        }
        SUCCESS
    }
}