use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::{hash_map::Entry, HashMap},
};

use crate::{
    ecs::{entities::Entity, errors::EcsErrors},
    logger::Logger,
};

use super::{
    comp_pool::{CompPool, GenericCompPool},
    Component,
};

pub type CellComponent<T> = RefCell<CompPool<T>>;

pub struct ComponentManager<'a> {
    component_pools: HashMap<TypeId, Box<dyn GenericCompPool + 'a>>,
    pub component_bit_masks: HashMap<TypeId, u32>,
    logger: Logger,
}

impl<'a> ComponentManager<'a> {
    pub fn new() -> Self {
        Self {
            component_pools: HashMap::new(),
            component_bit_masks: HashMap::new(),
            logger: Logger::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) -> &u32 {
        let comp_id = TypeId::of::<T>();

        if let Entry::Vacant(e) = self.component_pools.entry(comp_id) {
            e.insert(Box::new(RefCell::new(CompPool::<T>::new(30))));
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
                .set(entity.0, component)
                .unwrap();
        }
        self.component_bit_masks.get(&comp_id).unwrap()
    }

    pub fn remove<T: Component + 'static>(&mut self, entity: &Entity) -> Result<(), EcsErrors> {
        let comp_id = TypeId::of::<T>();
        if let Some(pool) = self.component_pools.get(&comp_id) {
            pool.as_any()
                .downcast_ref::<CellComponent<T>>()
                .unwrap()
                .borrow_mut()
                .remove(entity.0)
                .unwrap();

            Ok(())
        } else {
            Err(EcsErrors::component_does_not_exist::<T>())
        }
    }

    pub fn remove_all(&mut self, entity: &Entity) {
        self.component_pools
            .values_mut()
            .for_each(|pool| pool.remove_any(entity))
    }

    pub fn get_components<T: Component + 'static>(
        &self,
    ) -> Result<Ref<'_, CompPool<T>>, EcsErrors> {
        let comp_id = TypeId::of::<T>();
        if let Some(pool) = self.component_pools.get(&comp_id) {
            Ok(pool
                .as_any()
                .downcast_ref::<CellComponent<T>>()
                .unwrap()
                .borrow())
        } else {
            Err(EcsErrors::component_does_not_exist::<T>())
        }
    }

    pub fn get_components_mut<T: Component + 'static>(
        &self,
    ) -> Result<RefMut<'_, CompPool<T>>, EcsErrors> {
        let comp_id = TypeId::of::<T>();
        if let Some(pool) = self.component_pools.get(&comp_id) {
            Ok(pool
                .as_any()
                .downcast_ref::<CellComponent<T>>()
                .unwrap()
                .borrow_mut())
        } else {
            Err(EcsErrors::component_does_not_exist::<T>())
        }
    }

    pub fn get_mask<T: Component + 'static>(&self) -> Result<&u32, EcsErrors> {
        let comp_id = TypeId::of::<T>();
        if let Some(mask) = self.component_bit_masks.get(&comp_id) {
            Ok(mask)
        } else {
            Err(EcsErrors::component_does_not_exist::<T>())
        }
    }
}
