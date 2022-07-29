use std::collections::HashMap;
use crate::behavior::Knowledge;

use crate::World;
use crate::constants::MOB_SPEED;
use crate::behavior::btree::{BehaviorTreeNode, Status};
use crate::behavior::btree::Status::{RUNNING, SUCCESS};
use crate::components::{Position};
use crate::util::physics::Vect;
use crate::util::physics;
use crate::map::MapTile;


pub struct MoveToSpot {
    spot_x: f32,
    spot_y: f32,
}

impl BehaviorTreeNode for MoveToSpot {
    fn run(&mut self, state: &mut Knowledge, world: &World) -> Status {
        self.move_to(state, world)
    }
}

impl MoveToSpot {
    pub fn new() -> Self {
        Self { spot_x: 0.0, spot_y: 0.0 }
    }

    pub fn boxed(x: f32, y: f32) -> Box<Self> {
        Box::new(Self { spot_x: x, spot_y: y })
    }

    fn move_to(&self, state: &mut Knowledge, world: &World) -> Status {
        // let dest = state.destination.as_ref().unwrap(); // TODO check if dest is set

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let own_pos = positions.get_mut(state.owner).unwrap().as_mut().unwrap();

        if physics::distance_between(
            self.spot_x, self.spot_y,
            own_pos.x, own_pos.y,
        ) < 0.02 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &Vect::of(self.spot_x, self.spot_y),
            world.properties.delta_time);

        own_pos.x += velocity_vec.x;
        own_pos.y += velocity_vec.y;
        RUNNING
    }

    fn path(&mut self, world: &World) -> Status {
        world.map.set_tile_passable(5, 1, false);
        world.map.set_tile_passable(9, 3, false);

        let map_tiles = world.map.borrow_tiles();
        let start_tile = map_tiles.borrow_tile(1, 1);
        let dest_tile = map_tiles.borrow_tile(15, 15);

        let mut frontier = vec![start_tile];
        let mut came_from: HashMap<&MapTile, Option<&MapTile>> = Default::default();
        came_from.insert(start_tile, None);
        while !frontier.is_empty() {
            let current_tile = frontier.remove(0);

            if current_tile == dest_tile { break; }

            let neighbours = map_tiles.borrow_orthogonal_neighbors(current_tile);
            for i in 0..neighbours.len() {
                if !neighbours[i].passable { break; }
                if !came_from.contains_key(neighbours[i]) {
                    frontier.push(neighbours[i]);
                    came_from.insert(neighbours[i], Some(current_tile));
                }
            }
        }

        let mut current_tile = dest_tile;
        let mut path = vec![];
        while current_tile != start_tile {
            path.push(current_tile);
            current_tile = came_from.get(current_tile).unwrap().unwrap();
        }

        // println!("draw path");
        // for i in 0..path.len() {
        //     let path_node_id = world.ecs.create_entity();
        //     world.ecs.add_component_to_entity::<Position>(path_node_id,
        //                                                   Position::of(path[i].x as f32 + 0.5, path[i].y as f32 + 0.5, path_node_id));
        //     world.ecs.add_component_to_entity::<RenderShape>(path_node_id,
        //                                                      RenderShape::new_with_standard_offset(
        //                                                          0.4, 0.4,
        //                                                          Color::new(50, 50, 50, 255)));
        // }

        SUCCESS
    }
}

pub struct MoveCloseToTarget {}

impl BehaviorTreeNode for MoveCloseToTarget {
    fn run(&mut self, state: &mut Knowledge, world: &World) -> Status {
        self.move_close(state, world)
    }
}

impl MoveCloseToTarget {
    pub fn new() -> Self {
        Self {}
    }

    fn move_close(&self, state: &Knowledge, world: &World) -> Status {
        let target = state.target.expect(&*format!("Target is not set for {}", state.owner));

        let mut positions = world.ecs.borrow_component_vec_mut::<Position>();
        let target_pos = positions.get(target).unwrap().as_ref().unwrap();

        let destination = Vect::of(
            target_pos.x,
            target_pos.y,
        );

        let own_pos = positions.get_mut(state.owner).unwrap().as_mut().unwrap();

        if physics::distance_between_vect(
            &destination,
            &Vect::of(own_pos.x, own_pos.y),
        ) < 0.5 {
            return SUCCESS;
        }

        let velocity_vec = get_velocity_vec_to(
            &Vect::of(own_pos.x, own_pos.y),
            &destination,
            world.properties.delta_time);

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