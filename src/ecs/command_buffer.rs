use std::{any::TypeId, collections::VecDeque};

use super::{components::Component, entities::Entity};



pub enum WorldCommand {
    RemoveEntity(usize),
    RemoveComponent(usize, TypeId),
    CreateEntity(Vec<Box<dyn Component>>),
    AddComponent(usize, Box<dyn Component>),
}


pub struct CommandBuffer {
    commands: VecDeque<WorldCommand>
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new()
        }
    }

    pub fn remove_entity(&mut self, enitity: &Entity) {
        self.commands.push_front(WorldCommand::RemoveEntity(enitity.0));
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: &Entity) {
        self.commands.push_front(WorldCommand::RemoveComponent(entity.0, TypeId::of::<T>()));
    }

    pub fn add_component(&mut self, entity: &Entity, component: impl Component + 'static) {
        self.commands.push_front(WorldCommand::AddComponent(entity.0, Box::new(component)));
    }

    pub fn create_component(&mut self, components: Vec<Box<dyn Component>>) {
        self.commands.push_front(WorldCommand::CreateEntity(components));
    }

    pub fn iterate(&self) -> impl Iterator<Item = &WorldCommand> {
        self.commands.iter()
    }
}
