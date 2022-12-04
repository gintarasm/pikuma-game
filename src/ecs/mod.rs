use std::{
    any::TypeId,
    cell::RefMut,
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    process::Command,
};

use time::Duration;

use self::{command_buffer::CommandBuffer, components::Component, entities::Entity, query::Query, world::World, events::{WorldEventEmmiter, EventEmitter}};

pub mod command_buffer;
pub mod components;
pub mod entities;
pub mod errors;
pub mod query;
pub mod resources;
pub mod events;
mod tests;
pub mod world;

pub struct SystemBuilder<T: SystemAction + 'static> {
    comp_signatures: HashMap<TypeId, u32>,
    signature: u32,
    name: String,
    action: Box<dyn SystemAction>,
    phantom: PhantomData<T>,
}

impl<T: SystemAction + 'static> SystemBuilder<T> {
    pub fn new(name: &str, action: T, comp_signatures: HashMap<TypeId, u32>) -> Self {
        Self {
            comp_signatures,
            signature: 0,
            name: name.to_owned(),
            action: Box::new(action),
            phantom: PhantomData,
        }
    }

    pub fn with_component<C: Component + 'static>(mut self) -> Self {
        let comp_id = TypeId::of::<C>();
        let comp_sig = self.comp_signatures.get(&comp_id).unwrap();
        self.signature |= comp_sig;
        self
    }

    pub fn build(self) -> System {
        System {
            signature: self.signature,
            entities: Vec::new(),
            action: self.action,
            name: self.name,
        }
    }
}

pub struct System {
    pub name: String,
    pub signature: u32,
    entities: Vec<Entity>,
    action: Box<dyn SystemAction>,
}

impl System {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.retain(|e| e.0 != entity.0);
    }

    pub fn active(&mut self, world: &World) -> CommandBuffer {
        let mut buffer = CommandBuffer::new();
        let query = world.query();
        let emiter = world.emiter();
        self.action.action(query, &self.entities, &mut buffer, emiter);
        buffer
    }
}

pub trait SystemAction {
    fn action(&mut self, query: Query, entities: &Vec<Entity>, commands: &mut CommandBuffer, emitter: EventEmitter);
    fn to_system(self, world: &World) -> System;
}

