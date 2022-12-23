use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

use super::{
    components::{comp_pool::CompPool, component_manager::ComponentManager},
    entities::{entity_manager::EntityManager, Entity},
    resources::{Resource, Resources},
    Component,
};

pub struct Query<'a> {
    component_manager: &'a ComponentManager<'a>,
    entity_manager: &'a EntityManager<'a>,
    pub resources: &'a Resources,
}

pub struct ComponentQuery<'a> {
    component_manager: &'a ComponentManager<'a>,
}

pub struct EntityQuery<'a> {
    signature: u32,
    component_manager: &'a ComponentManager<'a>,
    entity_manager: &'a EntityManager<'a>,
}

impl<'a> Query<'a> {
    pub fn new(
        entity_manager: &'a EntityManager,
        component_manager: &'a ComponentManager,
        resources: &'a Resources,
    ) -> Self {
        Self {
            entity_manager,
            component_manager,
            resources,
        }
    }

    pub fn components(&self) -> ComponentQuery<'a> {
        ComponentQuery {
            component_manager: self.component_manager,
        }
    }

    pub fn entities(&self) -> EntityQuery<'a> {
        EntityQuery {
            signature: 0,
            entity_manager: self.entity_manager,
            component_manager: self.component_manager,
        }
    }

    pub fn resource<T: Any>(&self) -> Ref<Resource> {
        self.resources.get::<T>().borrow()
    }

    pub fn resource_mut<T: Any>(&self) -> RefMut<Resource> {
        self.resources.get::<T>().borrow_mut()
    }
}

pub struct QResource<'a, T> {
    r: Ref<'a, Resource>,
    p: PhantomData<T>,
}

impl<'a, T> QResource<'a, T> {
    pub fn get(&self) -> &'a T {
        self.r.get::<T>()
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

        self.entity_manager
            .entity_component_signatures
            .iter()
            .enumerate()
            .filter(|(_, sig)| (**sig & signature) == signature)
            .map(|(id, _)| Entity(id))
            .collect()
    }
}
