use std::{collections::HashMap, any::TypeId, cell::{RefCell, Ref, RefMut}};

use crate::{logger::Logger, ecs::entities::Entity};

use super::{comp_pool::{GenericCompPool, CompPool}, Component};


pub type CellComponent<T> = RefCell<CompPool<T>>;

pub struct ComponentManager<'a> {
    component_pools: HashMap<TypeId, Box<dyn GenericCompPool + 'a>>,
    pub component_bit_masks: HashMap<TypeId, u32>,
    logger: Logger,
}

impl<'a> ComponentManager<'a> {
    pub fn new() -> Self {
        Self { component_pools: HashMap::new(), component_bit_masks: HashMap::new(), logger: Logger::new() }
    }
    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) -> u32 {
        let comp_id = TypeId::of::<T>();

        if !self.component_pools.contains_key(&comp_id) {
            self.component_pools
                .insert(comp_id, Box::new(RefCell::new(CompPool::<T>::new(30))));
            let current_count = self.component_bit_masks.len();
            self.component_bit_masks.insert(comp_id, 1 << current_count);
        }

        if let Some(pool) = self.component_pools.get_mut(&comp_id) {
            if pool.get_size() <= entity.0 {
                pool.resize(entity.0 + 1);
            }

            pool.as_any()
                .downcast_ref::<CellComponent<T>>()
                .unwrap()
                .borrow_mut()
                .set(entity.0, component);
        }
        self.component_bit_masks.get(&comp_id).unwrap().clone()
    }
    
    pub fn remove<T: Component + 'static>(&mut self, entity: &Entity) {
        let comp_id = TypeId::of::<T>();
        self.component_pools
            .get(&comp_id)
            .unwrap()
            .as_any()
            .downcast_ref::<CellComponent<T>>()
            .unwrap()
            .borrow_mut()
            .remove(entity.0);
    }

    pub fn remove_all(&mut self, entity: &Entity) {
        self.component_pools.values_mut().for_each(|pool| pool.remove_any(entity))
    }

    pub fn get_components<T: Component + 'static>(&self) -> Option<Ref<'_, CompPool<T>>> {
        let comp_id = TypeId::of::<T>();
        self.component_pools
            .get(&comp_id)?
            .as_any()
            .downcast_ref::<CellComponent<T>>()
            .map(|s| s.borrow())
    }
    
    pub fn get_components_mut<T: Component + 'static>(&self) -> Option<RefMut<'_, CompPool<T>>> {
        let comp_id = TypeId::of::<T>();
        self.component_pools
            .get(&comp_id)?
            .as_any()
            .downcast_ref::<CellComponent<T>>()
            .map(|s| s.borrow_mut())
    }

    pub fn get_mask<T: Component + 'static>(&self) -> Option<u32> {
        let comp_id = TypeId::of::<T>();
        self.component_bit_masks.get(&comp_id).map(|i| i.clone())
    }
}
