use crate::btree::{BehaviorTreeNode, Status};
use crate::components::{Position, TargetEntity, TargetPosition};
use crate::physics::{Vect};
use crate::{physics, World};
use crate::btree::Status::{RUNNING, SUCCESS};
use crate::constants::MOB_SPEED;

pub struct MoveToPositionTask {
    owner_id: usize,
}

impl BehaviorTreeNode for MoveToPositionTask {
    fn run(&mut self, world: &World) -> Status {
        self.move_to(world)
    }
}

impl MoveToPositionTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn move_to(&self, world: &World) -> Status {
        let target_positions = world.ecs.borrow_component_vec::<TargetPosition>();
        let target_pos = target_positions.get(self.owner_id).unwrap().as_ref().unwrap();

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let own_pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if physics::distance_between(&Vect::of(target_pos.x, target_pos.y), &Vect::of(own_pos.x, own_pos.y)) < 0.02 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &Vect::of(target_pos.x, target_pos.y),
            world.delta_time);

        own_pos.x += velocity_vec.x;
        own_pos.y += velocity_vec.y;
        RUNNING
    }
}

pub struct MoveCloseToTargetTask {
    owner_id: usize,
}

impl BehaviorTreeNode for MoveCloseToTargetTask {
    fn run(&mut self, world: &World) -> Status {
        self.move_close(world)
    }
}

impl MoveCloseToTargetTask {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn move_close(&self, world: &World) -> Status {
        let targets = world.ecs.borrow_component_vec::<TargetEntity>();
        let target_id = targets.get(self.owner_id).unwrap().as_ref().unwrap().target_id;

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();

        let destination = Vect::of(
            positions.get(target_id).unwrap().as_ref().unwrap().x,
            positions.get(target_id).unwrap().as_ref().unwrap().y,
        );

        let own_pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if physics::distance_between(&destination, &Vect::of(own_pos.x, own_pos.y)) < 0.5 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &destination,
            world.delta_time);

        own_pos.x += velocity_vec.x;
        own_pos.y += velocity_vec.y;
        RUNNING
    }
}

fn get_velocity_vec_to(start_pos: &Vect, destination: &Vect, delta_time: f32) -> Vect {
    let mut velocity = physics::vector_to(start_pos, destination);
    velocity.normalize();
    velocity.set_length(velocity.length() * delta_time * MOB_SPEED);
    velocity
}