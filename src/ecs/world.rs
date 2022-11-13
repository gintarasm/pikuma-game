use crate::logger::Logger;
use std::{
    any::{type_name, TypeId},
    collections::{HashMap, HashSet},
};

use super::comp_pool::{CompPool, GenericCompPool};
use super::{Component, Entity, System, SystemAction};

pub struct World {
    num_of_entities: usize,
    component_pools: HashMap<TypeId, Box<dyn GenericCompPool>>,
    entity_component_signatures: Vec<HashSet<TypeId>>,
    systems: HashMap<TypeId, System>,

    entities_to_add: HashSet<Entity>,
    entities_to_remove: HashSet<Entity>,
    logger: Logger,
}

impl World {
    pub fn new() -> Self {
        Self {
            num_of_entities: 0,
            component_pools: HashMap::new(),
            entity_component_signatures: Vec::new(),
            systems: HashMap::new(),
            entities_to_add: HashSet::new(),
            entities_to_remove: HashSet::new(),
            logger: Logger::new(),
        }
    }

    pub fn update(&mut self) {
        let entities_to_add = std::mem::take(&mut self.entities_to_add);
        entities_to_add.iter().for_each(|entity| {
            self.add_entity_to_systems(*entity);
        });
        self.entities_to_add.clear();
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.num_of_entities;
        self.num_of_entities += 1;

        self.entities_to_add.insert(Entity(entity_id));

        if entity_id > self.entity_component_signatures.len() {
            self.entity_component_signatures
                .resize_with(entity_id + 1, || HashSet::new())
        }

        self.logger
            .info(&format!("Entity created with id = {entity_id}"));

        Entity(entity_id)
    }

    fn add_entity_to_systems(&mut self, entity: Entity) {
        let key = self.entity_component_signatures.get(entity.0).unwrap();
        self.systems.values_mut().for_each(|system| {
            if system.signature.eq(key) {
                self.logger.info(&format!(
                    "Adding entity id = {} to system {}",
                    entity.0, system.name
                ));
                system.add_entity(entity.clone());
            }
        });
    }

    pub fn kill_entity(&mut self, entity: &Entity) {
        self.logger
            .info(&format!("Killing entity id = {}", entity.0));
        self.entities_to_remove.insert(*entity);
    }

    pub fn add_system<T: SystemAction + 'static>(&mut self, system: System) {
        let system_id = TypeId::of::<T>();
        if let Some(system) = self.systems.insert(system_id, system) {
            self.logger.info(&format!("Adding systems {}", system.name));
        }
    }

    pub fn remove_system<T: SystemAction + 'static>(&mut self) {
        let system_id = TypeId::of::<T>();
        if let Some(system) = self.systems.remove(&system_id) {
            self.logger
                .info(&format!("Removing system {}", system.name));
        }
    }

    pub fn has_system<T: SystemAction + 'static>(&self) -> bool {
        let system_id = TypeId::of::<T>();
        self.systems.contains_key(&system_id)
    }

    pub fn get_system<T: SystemAction + 'static>(&self) -> &System {
        let system_id = TypeId::of::<T>();
        self.systems.get(&system_id).unwrap()
    }

    pub fn get_system_mut<T: SystemAction + 'static>(&mut self) -> &mut System {
        let system_id = TypeId::of::<T>();
        self.systems.get_mut(&system_id).unwrap()
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        let comp_id = TypeId::of::<T>();

        if !self.component_pools.contains_key(&comp_id) {
            self.component_pools
                .insert(comp_id, Box::new(CompPool::<T>::new(30)));
        }

        if let Some(pool) = self.component_pools.get_mut(&comp_id) {
            if pool.get_size() <= entity.0 {
                pool.resize(self.num_of_entities);
            }

            pool.as_any_mut()
                .downcast_mut::<CompPool<T>>()
                .unwrap()
                .set(entity.0, component);
            if let None = self.entity_component_signatures.get(entity.0) {
                self.entity_component_signatures
                    .resize_with(entity.0 + 1, || HashSet::new())
            }
            self.entity_component_signatures
                .get_mut(entity.0)
                .unwrap()
                .insert(comp_id);

            self.logger.info(&format!(
                "Add component {} to Entity Id = {}",
                type_name::<T>(),
                entity.0
            ));
        }
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: &Entity) {
        let comp_id = TypeId::of::<T>();
        self.entity_component_signatures
            .get_mut(entity.0)
            .unwrap()
            .remove(&comp_id);

        self.logger.info(&format!("Removing component {} from Entity Id = {}", type_name::<T>(), entity.0));
    }

    pub fn has_component<T: Component + 'static>(&self, entity: &Entity) -> bool {
        let comp_id = TypeId::of::<T>();
        self.entity_component_signatures
            .get(entity.0)
            .unwrap()
            .contains(&comp_id)
    }

    pub fn get_component<T: Component + 'static>(&self, entity: &Entity) -> Option<&T> {
        let comp_id = TypeId::of::<T>();

        self.component_pools
            .get(&comp_id)?
            .as_any()
            .downcast_ref::<CompPool<T>>()?
            .get(entity.0)
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        let comp_id = TypeId::of::<T>();

        self.component_pools
            .get_mut(&comp_id)?
            .as_any_mut()
            .downcast_mut::<CompPool<T>>()?
            .get_mut(entity.0)
    }
}
