use std::{collections::HashMap, any::TypeId};

use crate::{logger::Logger, ecs::entities::Entity};

use super::{comp_pool::{GenericCompPool, CompPool}, Component};



pub struct ComponentManager {
    component_pools: HashMap<TypeId, Box<dyn GenericCompPool>>,
    pub component_bit_masks: HashMap<TypeId, u32>,
    logger: Logger,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self { component_pools: HashMap::new(), component_bit_masks: HashMap::new(), logger: Logger::new() }
    }
    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) -> u32 {
        let comp_id = TypeId::of::<T>();

        if !self.component_pools.contains_key(&comp_id) {
            self.component_pools
                .insert(comp_id, Box::new(CompPool::<T>::new(30)));
            let current_count = self.component_bit_masks.len();
            self.component_bit_masks.insert(comp_id, 1 << current_count);
        }

        if let Some(pool) = self.component_pools.get_mut(&comp_id) {
            if pool.get_size() <= entity.0 {
                pool.resize(entity.0 + 1);
            }

            pool.as_any_mut()
                .downcast_mut::<CompPool<T>>()
                .unwrap()
                .set(entity.0, component);
        }
        self.component_bit_masks.get(&comp_id).unwrap().clone()
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

    pub fn get_mask<T: Component + 'static>(&self) -> Option<u32> {
        let comp_id = TypeId::of::<T>();
        self.component_bit_masks.get(&comp_id).map(|i| i.clone())
    }
}