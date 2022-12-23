use time::Duration;

use crate::{ecs::command_buffer::WorldCommand, logger::Logger};
use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use super::{
    command_buffer::{self, CommandBuffer},
    components::Component,
    entities::{self, entity_manager::EntityManager, Entity},
    events::{EventEmitter, GameEvent, WorldEventEmmiter, WorldEventSubscriber, WorldEvents},
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

    current_entity: Option<Entity>,
    events: WorldEvents,
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
            current_entity: None,
            events: WorldEvents::new(),
        }
    }

    pub fn update(&mut self) {
        let entities_to_add = std::mem::take(&mut self.entities_to_add);
        entities_to_add.iter().for_each(|entity| {
            self.add_entity_to_systems(*entity);
        });

        let entities_to_remove = std::mem::take(&mut self.entities_to_remove);
        entities_to_remove
            .iter()
            .for_each(|entity| self.kill_entity(entity));
    }

    pub fn create_entity(&mut self) -> &mut Self {
        let entity = self.entity_manager.create_entity();

        self.current_entity = Some(entity);
        self.entities_to_add.insert(entity);

        self
    }

    pub fn with_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        let entity = self.current_entity.unwrap();
        self.add_component(&entity, component);
        self
    }

    pub fn finish_entity(&mut self) -> Entity {
        self.current_entity.unwrap()
    }

    fn add_entity_to_systems(&mut self, entity: Entity) {
        self.logger
            .info(&format!("Adding entity id = {} to systems", entity.0));

        let key = self.entity_manager.get_signature(&entity).unwrap();

        self.systems
            .values_mut()
            .filter(|s| (*key & s.signature) == s.signature)
            .for_each(|system| {
                system.add_entity(entity);
                self.logger.info(&format!(
                    "Adding entity id = {} to system {}",
                    entity.0, system.name
                ));
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

        let key = self.entity_manager.get_signature(entity).unwrap();
        self.systems
            .values_mut()
            .filter(|s| (*key & s.signature) == s.signature)
            .for_each(|system| {
                self.logger.info(&format!(
                    "Removing id = {} from system {}",
                    entity.0, system.name
                ));
                system.remove_entity(entity);
            });

        self.entity_manager.remove_entity(entity);
    }

    pub fn events(&mut self) -> &mut impl WorldEventSubscriber {
        &mut self.events
    }

    pub fn emiter(&self) -> EventEmitter {
        self.events.emiter()
    }

    pub fn emit_event<T: GameEvent + 'static>(&mut self, event: T) {
        let mut cmd_buffer = CommandBuffer::new();
        let query = self.query();

        self.emiter().emit(event, &mut cmd_buffer, &query);
        self.handle_commands(cmd_buffer);
    }

    pub fn add_system<T: SystemAction + 'static>(&mut self, system_action: T, update: bool) {
        let system_id = TypeId::of::<T>();
        let mut system = system_action.to_system(self);
        let signature = system.signature.clone();
        if update {
            self.entity_manager
                .entity_component_signatures
                .iter()
                .enumerate()
                .filter(|(_, s)| (*s & signature) == signature)
                .for_each(|(id, _)| system.add_entity(Entity(id)));
        }
        self.logger.info(&format!("Adding systems {}", system.name));
        self.systems.insert(system_id, system);
        println!("systems add {:?}", self.systems.len());
    }

    pub fn remove_system<T: SystemAction + 'static>(&mut self) {
        let system_id = TypeId::of::<T>();
        if let Some(system) = self.systems.remove(&system_id) {
            self.logger
                .info(&format!("Removing system {}", system.name));
        }
    }

    pub fn update_system<T: SystemAction + 'static>(&mut self) {
        let system_id = TypeId::of::<T>();
        let mut systems = std::mem::take(&mut self.systems);
        if let Some(system) = systems.get_mut(&system_id) {
            self.logger
                .info(&format!("Updating system {}", system.name));

            let command_buffer = system.active(self);
            self.handle_commands(command_buffer);
        } else {
            self.logger
                .info(&format!("Skipping system {} update", type_name::<T>()));
        }

        self.systems = std::mem::take(&mut systems);
    }

    fn handle_commands(&mut self, command_buffer: CommandBuffer) {
        for command in command_buffer.iterate() {
            match command {
                WorldCommand::RemoveEntity(id) => self.remove_entity(&Entity(*id)),
                WorldCommand::RemoveComponent(id, comp_id) => {
                    self.remove_component_with_id(&Entity(*id), comp_id)
                }
                WorldCommand::AddComponent(id, comp) => todo!(),
                WorldCommand::CreateEntity(components) => todo!(),
            }
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

    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.delete::<T>();
        self.logger
            .info(&format!("Deleting resource {}", type_name::<T>()));
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        self.entity_manager
            .add_component(entity, component)
            .unwrap();

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

    fn remove_component_with_id(&mut self, entity: &Entity, comp_id: &TypeId) {
        self.entity_manager.remove_component_for_id(entity, comp_id);
        self.logger.info(&format!(
            "Removing component {} from Entity Id = {}",
            "Unknown", entity.0
        ));
    }

    pub fn has_component<T: Component + 'static>(&self, entity: &Entity) -> bool {
        self.entity_manager.has_component::<T>(entity).unwrap()
    }

    pub fn get_component_signatures(&self) -> HashMap<TypeId, u32> {
        self.entity_manager.get_component_signatures()
    }

    pub fn query(&self) -> Query {
        Query::new(
            &self.entity_manager,
            &self.entity_manager.component_manager,
            &self.resources,
        )
    }
}
