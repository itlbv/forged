use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{RUNNING, SUCCESS};
use crate::components::Position;
use crate::World;
use crate::constants::MOB_SPEED;
use crate::physics::{distance_between, Vec, vector_to};

pub struct MoveTask {
    owner_id: usize,
    destination: Vec,
}

impl BehaviorTreeNode for MoveTask {
    fn run(&self, world: &World) -> Status {
        self.move_to(world)
    }
}

impl MoveTask {
    pub fn new(owner_id: usize, x: f32, y: f32) -> Self {
        Self { owner_id, destination: Vec { x, y } }
    }

    fn move_to(&self, world: &World) -> Status {
        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if distance_between(&self.destination, &Vec::of(pos.x, pos.y)) < 0.02 {
            return SUCCESS;
        }

        let mut direction = vector_to(&Vec::of(pos.x, pos.y), &self.destination);
        direction.normalize();
        direction.set_length(direction.length() * world.delta_time * MOB_SPEED);

        pos.x += direction.x;
        pos.y += direction.y;
        RUNNING
    }
}