use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::ecs::components::component_manager::ComponentManager;
use crate::{ecs::components::Component, logger::Logger};

use super::Entity;

struct EntityIdGenerator {
    num_of_entities: usize,
    freed_entities: Vec<usize>,
}

impl EntityIdGenerator {
    pub fn new() -> Self {
        Self {
            num_of_entities: 0,
            freed_entities: vec![],
        }
    }

    pub fn get_id(&mut self) -> usize {
        if self.freed_entities.is_empty() {
            let id = self.num_of_entities;
            self.num_of_entities += 1;
            id
        } else {
            self.freed_entities.pop().unwrap()
        }
    }

    pub fn free_id(&mut self, id: usize) {
        self.freed_entities.push(id)
    }
}

pub struct EntityManager<'a> {
    id_generator: EntityIdGenerator,
    pub entity_component_signatures: Vec<u32>,
    pub component_manager: ComponentManager<'a>,
    logger: Logger,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> Self {
        Self {
            id_generator: EntityIdGenerator::new(),
            entity_component_signatures: vec![],
            component_manager: ComponentManager::new(),
            logger: Logger::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.id_generator.get_id();

        if entity_id >= self.entity_component_signatures.len() {
            self.entity_component_signatures.resize(entity_id + 10, 0);
        } else {
            self.entity_component_signatures[entity_id] = 0;
        }

        self.logger
            .info(&format!("Entity created with id = {entity_id}"));

        Entity(entity_id)
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.logger
            .info(&format!("Removing entity id = {}", entity.0));

        self.entity_component_signatures[entity.0] = 0;
        self.component_manager.remove_all(entity);
        self.id_generator.free_id(entity.0);
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        let comp_mask = self.component_manager.add_component(entity, component);

        if let None = self.entity_component_signatures.get(entity.0) {
            self.entity_component_signatures.resize(entity.0 + 1, 0);
        }

        self.entity_component_signatures[entity.0] |= comp_mask;

        self.logger.info(&format!(
            "Add component {} to Entity Id = {}",
            type_name::<T>(),
            entity.0
        ));
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: &Entity) {
        let comp_mask = self.component_manager.get_mask::<T>().unwrap();
        self.entity_component_signatures[entity.0] &= !comp_mask;
        self.component_manager.remove::<T>(entity);

        self.logger.info(&format!(
            "Removing component {} from Entity Id = {}",
            type_name::<T>(),
            entity.0
        ));
    }

    pub fn has_component<T: Component + 'static>(&self, entity: &Entity) -> bool {
        let comp_mask = self.component_manager.get_mask::<T>().unwrap();

        let signature = self.entity_component_signatures.get(entity.0).unwrap();

        (signature & comp_mask) == comp_mask
    }

    pub fn get_signature(&self, entity: &Entity) -> Option<u32> {
        self.entity_component_signatures
            .get(entity.0)
            .map(|i| i.clone())
    }

    pub fn get_component_signatures(&self) -> HashMap<TypeId, u32> {
        self.component_manager.component_bit_masks.clone()
    }
}
