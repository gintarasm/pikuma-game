use crate::logger::Logger;
use std::{
    any::{type_name, Any, TypeId},
    collections::{HashMap, HashSet},
};

use super::{
    components::Component,
    entities::{entity_manager::EntityManager, Entity},
    query::Query,
    resources::Resources,
};
use super::{System, SystemAction};

pub struct World<'a> {
    entity_manager: EntityManager<'a>,
    systems: HashMap<TypeId, System>,
    resources: Resources,

    entities_to_add: HashSet<Entity>,
    entities_to_remove: HashSet<Entity>,
    logger: Logger,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            systems: HashMap::new(),
            resources: Resources::new(),
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

        let entities_to_remove = std::mem::take(&mut self.entities_to_remove);
        entities_to_remove
            .iter()
            .for_each(|entity| self.kill_entity(entity));
        self.entities_to_remove.clear();
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = self.entity_manager.create_entity();

        self.entities_to_add.insert(entity);

        entity
    }

    fn add_entity_to_systems(&mut self, entity: Entity) {
        self.logger
            .info(&format!("Adding entity id = {} to systems", entity.0));

        let key = self.entity_manager.get_signature(&entity).unwrap();

        self.systems
            .values_mut()
            .filter(|s| s.signature == *key)
            .for_each(|system| {
                self.logger.info(&format!(
                    "Adding entity id = {} to system {}",
                    entity.0, system.name
                ));
                system.add_entity(entity);
            });
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.logger
            .info(&format!("Removing entity id = {}", entity.0));

        self.entities_to_remove.insert(*entity);
    }

    fn kill_entity(&mut self, entity: &Entity) {
        self.logger
            .info(&format!("Killing entity id = {}", entity.0));

        let key = self.entity_manager.get_signature(&entity).unwrap().clone();
        self.entity_manager.remove_entity(entity);
        self.systems
            .values_mut()
            .filter(|s| s.signature == key)
            .for_each(|system| {
                self.logger.info(&format!(
                    "Removing id = {} from system {}",
                    entity.0, system.name
                ));
                system.remove_entity(entity);
            });
    }

    pub fn add_system<T: SystemAction + 'static>(&mut self, system_action: T) {
        let system_id = TypeId::of::<T>();
        let system = system_action.to_system(self);
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

    pub fn add_resource<T: Any>(&mut self, resource: T) {
        self.resources.add(resource);
        self.logger
            .info(&format!("Add resource {}", type_name::<T>(),));
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_ref_mut()
    }

    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.delete::<T>();
        self.logger
            .info(&format!("Deleting resource {}", type_name::<T>()));
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        self.entity_manager.add_component(entity, component).unwrap();

        self.logger.info(&format!(
            "Add component {} to Entity Id = {}",
            type_name::<T>(),
            entity.0
        ));
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: &Entity) {
        self.entity_manager.remove_component::<T>(entity).unwrap();
        self.logger.info(&format!(
            "Removing component {} from Entity Id = {}",
            type_name::<T>(),
            entity.0
        ));
    }

    pub fn has_component<T: Component + 'static>(&self, entity: &Entity) -> bool {
        self.entity_manager.has_component::<T>(entity).unwrap()
    }

    pub fn get_component_signatures(&self) -> HashMap<TypeId, u32> {
        self.entity_manager.get_component_signatures()
    }

    pub fn query(&self) -> Query {
        Query::new(&self.entity_manager, &self.entity_manager.component_manager)
    }
}
