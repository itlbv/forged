use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::components::{Food, Position};
use crate::World;
use crate::constants::MOB_SPEED;
use crate::physics::{distance_between, Vect, vector_to};

pub struct DoNothingTask {}

impl BehaviorTreeNode for DoNothingTask {
    fn run(&self, world: &World) -> Status {
        SUCCESS
    }
}

pub struct FindFoodTask {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindFoodTask {
    fn run(&self, world: &World) -> Status {
        self.find_food(world)
    }
}

impl FindFoodTask {
    fn find_food(&self, world: &World) -> Status {
        let foods = world.ecs.borrow_component_vec::<Food>();
        let positions = world.ecs.borrow_component_vec::<Position>();

        let own_pos_vect = Vect::of(
            positions.get(self.owner_id).unwrap().as_ref().unwrap().x,
            positions.get(self.owner_id).unwrap().as_ref().unwrap().y,
        );

        let zip = foods.iter().zip(positions.iter());
        let iter = zip.filter_map(
            |(food, pos)| Some((food.as_ref()?, pos.as_ref()?))
        );

        let mut target_entity_id: i32 = -1;
        let mut shortest_distance: f32 = 10000.0;
        let mut target_x = 0.0;
        let mut target_y = 0.0;
        for (food, pos) in iter {
            let distance = distance_between(&own_pos_vect, &Vect::of(pos.x, pos.y));
            if distance < shortest_distance {
                shortest_distance = distance;
                target_entity_id = pos.entity_id as i32;
                target_x = pos.x;
                target_y = pos.y;
            }
        }

        if target_entity_id < 0 {
            println!("No food found");
            return FAILURE;
        }

        println!("Closest food is at: {}, {}", target_x, target_y);
        SUCCESS
    }
}

pub struct MoveTask {
    owner_id: usize,
    destination: Vect,
}

impl BehaviorTreeNode for MoveTask {
    fn run(&self, world: &World) -> Status {
        self.move_to(world)
    }
}

impl MoveTask {
    pub fn new(owner_id: usize, x: f32, y: f32) -> Self {
        Self { owner_id, destination: Vect { x, y } }
    }

    fn move_to(&self, world: &World) -> Status {
        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if distance_between(&self.destination, &Vect::of(pos.x, pos.y)) < 0.02 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(pos.x, pos.y),
            &self.destination,
            world.delta_time);

        pos.x += velocity_vec.x;
        pos.y += velocity_vec.y;
        RUNNING
    }
}

pub struct MoveCloseTask {
    owner_id: usize,
    destination: Vect,
}

impl BehaviorTreeNode for MoveCloseTask {
    fn run(&self, world: &World) -> Status {
        self.move_close(world)
    }
}

impl MoveCloseTask {
    fn move_close(&self, world: &World) -> Status {
        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let pos = positions.get_mut(self.owner_id).unwrap().as_mut().unwrap();

        if distance_between(&self.destination, &Vect::of(pos.x, pos.y)) < 1.0 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(pos.x, pos.y),
            &self.destination,
            world.delta_time);

        pos.x += velocity_vec.x;
        pos.y += velocity_vec.y;
        RUNNING
    }
}

fn get_velocity_vec_to(start_pos: &Vect, destination: &Vect, delta_time: f32) -> Vect {
    let mut velocity = vector_to(start_pos, destination);
    velocity.normalize();
    velocity.set_length(velocity.length() * delta_time * MOB_SPEED);
    velocity
}