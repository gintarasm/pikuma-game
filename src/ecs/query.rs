use std::{any::Any, cell::{Ref, RefMut}};

use super::{Component, components::{comp_pool::CompPool, component_manager::ComponentManager}, entities::{Entity, entity_manager::EntityManager}, resources::Resources};

pub struct Query<'a> {
    component_manager: &'a ComponentManager<'a>,
    entity_manager: &'a EntityManager<'a>,
    resources: &'a Resources,
}

pub struct ComponentQuery<'a> {
    component_manager: &'a ComponentManager<'a>,
}

pub struct EntityQuery<'a> {
    signature: u32,
    component_manager: &'a ComponentManager<'a>,
    entity_manager: &'a EntityManager<'a>
}

pub struct ResourceQuery<'a> {
    resources: &'a Resources
}


impl<'a> Query<'a> {
    pub fn new(entity_manager: &'a EntityManager, component_manager: &'a ComponentManager, resources: &'a Resources) -> Self {
        Self { entity_manager, component_manager, resources }
    }

    pub fn components(&self) -> ComponentQuery<'a> {
        ComponentQuery {
            component_manager: self.component_manager,
        }
    }

    pub fn entities(&self) -> EntityQuery<'a> {
        EntityQuery { signature: 0, entity_manager: self.entity_manager, component_manager: self.component_manager }
    }

    pub fn resources(&self) -> ResourceQuery<'a> {
        ResourceQuery { resources: self.resources }
    }
}

impl<'a> ComponentQuery<'a> {
    pub fn get<T: Component + 'static>(self) -> Ref<'a, CompPool<T>> {
        self.component_manager.get_components::<T>().unwrap()
    }

    pub fn get_mut<T: Component + 'static>(self) -> RefMut<'a, CompPool<T>> {
        self.component_manager.get_components_mut::<T>().unwrap()
    }
}

impl<'a> EntityQuery<'a> {
    pub fn with_component<T: Component + 'static>(mut self) -> Self {
        let sig = self.component_manager.get_mask::<T>().unwrap();
        self.signature |= sig;
        self
    }

    pub fn get(self) -> Vec<Entity> {
        let signature = self.signature;

        self.entity_manager.entity_component_signatures
            .iter()
            .enumerate()
            .filter(|(_, sig)| (**sig & signature) == signature)
            .map(|(id, _)| Entity(id))
            .collect()
    }
}

impl <'a> ResourceQuery<'a> {
    pub fn get<T: Any + 'static>(self) -> &'a T {
        self.resources.get_ref::<T>().unwrap()
    }
}
