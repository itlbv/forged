use std::any::Any;
use std::collections::HashMap;
use crate::behavior::btree::BehaviorTreeNode;
use crate::behavior::commands::{CalculateStrategy, Command};
use crate::ecs::EntityId;
use crate::util::physics::Vect;
use anyhow::{anyhow, Context, Error, Result};

pub struct Brain {
    pub knowledge: Knowledge,
    pub commands: Vec<Box<dyn Command>>,
    pub behaviors: Vec<Box<dyn BehaviorTreeNode>>,
}

impl Brain {
    pub fn new(owner: EntityId) -> Self {
        Self {
            knowledge: Knowledge::new(owner),
            commands: vec![CalculateStrategy::boxed()],
            behaviors: vec![],
        }
    }
}

pub struct Knowledge {
    pub owner: EntityId,
    pub target: Option<EntityId>,
    pub main_target: Option<EntityId>,
    pub destination: Option<Vect>,
    knowledge_map: HashMap<String, Box<dyn Any>>,
}

impl Knowledge {
    pub fn new(owner: EntityId) -> Self {
        Self {
            owner,
            target: None,
            main_target: None,
            destination: None,
            knowledge_map: HashMap::new(),
        }
    }

    pub fn put(&mut self, k: &str, v: Box<dyn Any>) {
        self.knowledge_map.insert(k.to_string(), v);
    }

    pub fn get<T: 'static>(&self, k: &str) -> Result<&T> {
        let result = self.knowledge_map
            .get(k)
            .with_context(|| format!("No element named {} in the knowledge of {}", k, self.owner))?
            .downcast_ref::<T>()
            .with_context(|| format!("Can't downcast element {} in the knowledge of {}", k, self.owner))?;
        Ok(result)
    }
}