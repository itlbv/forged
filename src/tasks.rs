use crate::btree::{BehaviorTreeNode, Status};
use crate::btree::Status::{FAILURE, RUNNING, SUCCESS};
use crate::components::{Food, Position, Recipe, Remove, Target, Destination};
use crate::physics::{distance_between, Vect};
use crate::World;

pub struct SetDestinationFromMainTarget {
    pub owner_id: usize,
}

impl BehaviorTreeNode for SetDestinationFromMainTarget {
    fn run(&mut self, world: &World) -> Status {
        self.set_destination(world)
    }
}

impl SetDestinationFromMainTarget {
    pub fn new(owner_id: usize) -> Self { Self { owner_id } }

    fn set_destination(&self, world: &World) -> Status {
        world.ecs.add_component_to_entity(self.owner_id, Destination::new(5.5, 7.5));
        SUCCESS
    }
}

pub struct DoUntilFailure {
    pub children: Vec<Box<dyn BehaviorTreeNode>>,
    idx: i8,
}

impl DoUntilFailure {
    pub fn of(children: Vec<Box<dyn BehaviorTreeNode>>) -> Self {
        Self { children, idx: -1 }
    }
}

impl BehaviorTreeNode for DoUntilFailure {
    fn run(&mut self, world: &World) -> Status {
        loop {
            for (i, child) in self.children.iter_mut().enumerate() {
                if self.idx >= 0 && self.idx != i as i8 { continue; }

                let status = child.run(world);
                if status == SUCCESS {
                    self.idx = -1;
                    continue;
                } else if status == RUNNING {
                    self.idx = i as i8;
                    return RUNNING;
                } else if status == FAILURE {
                    self.idx = -1;
                    return SUCCESS;
                }
            }
        }
    }
}

pub struct SetRecipe {
    own_id: usize,
    recipe: Recipe,
}

impl BehaviorTreeNode for SetRecipe {
    fn run(&mut self, world: &World) -> Status {
        self.set_recipe(world)
    }
}

impl SetRecipe {
    pub fn new(own_id: usize, recipe: Recipe) -> Self {
        Self { own_id, recipe }
    }

    fn set_recipe(&self, world: &World) -> Status {
        world.ecs.add_component_to_entity(self.own_id, self.recipe.clone());
        SUCCESS
    }
}

pub struct DoNothingTask {}

impl BehaviorTreeNode for DoNothingTask {
    fn run(&mut self, _: &World) -> Status {
        SUCCESS
    }
}

impl DoNothingTask {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct FindFood {
    pub owner_id: usize,
}

impl BehaviorTreeNode for FindFood {
    fn run(&mut self, world: &World) -> Status {
        self.find_food(world)
    }
}

impl FindFood {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

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
        for (_food, pos) in iter {
            let distance = distance_between(&own_pos_vect, &Vect::of(pos.x, pos.y));
            if distance < shortest_distance {
                shortest_distance = distance;
                target_entity_id = pos.entity_id as i32;
            }
        }

        if target_entity_id < 0 {
            println!("No food found");
            return FAILURE;
        }

        world.ecs.add_component_to_entity(self.owner_id, Target::new(target_entity_id as usize));
        SUCCESS
    }
}

pub struct EatTarget {
    owner_id: usize,
}

impl BehaviorTreeNode for EatTarget {
    fn run(&mut self, world: &World) -> Status {
        self.eat(world)
    }
}

impl EatTarget {
    pub fn new(owner_id: usize) -> Self {
        Self { owner_id }
    }

    fn eat(&self, world: &World) -> Status {
        println!("eat food");
        let targets = world.ecs.borrow_component_vec::<Target>();
        let target_id = targets.get(self.owner_id).unwrap().as_ref().unwrap().target_id;
        world.ecs.add_component_to_entity::<Remove>(target_id, Remove::new(target_id));
        SUCCESS
    }
}
