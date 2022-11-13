use std::{
    any::{Any, TypeId},
    collections::HashSet,
};

use self::world::World;

pub mod comp_pool;
pub mod world;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
pub struct Entity(usize);
pub struct SystemBuilder {
    name: String,
    action: Box<dyn SystemAction>,
    components: HashSet<TypeId>,
}

impl SystemBuilder {
    pub fn new<T: SystemAction>(name: &str, action: T) -> Self {
        Self {
            name: name.to_owned(),
            action: Box::new(action),
            components: HashSet::new(),
        }
    }

    pub fn with_component<T: Component + ?Sized + Any>(mut self) -> Self {
        let id = TypeId::of::<T>();
        self.components.insert(id);
        self
    }

    pub fn build(self) -> System {
        System {
            signature: self.components,
            entities: Vec::new(),
            action: self.action,
            name: self.name
        }
    }
}

pub struct System {
    pub name: String,
    pub signature: HashSet<TypeId>,
    entities: Vec<Entity>,
    action: Box<dyn SystemAction>,
}

impl System {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.retain(|e| e.0 != entity.0);
    }

    pub fn active(&self, world: &mut World) {
        self.action.action(world, &self.entities);
    }
}

pub trait SystemAction {
    fn action(&self, world: &mut World, entities: &Vec<Entity>);
    fn to_system(self) -> System;
}

pub trait Component {}
