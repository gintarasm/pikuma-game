use std::{any::TypeId, collections::HashMap, cell::{RefCell, Ref, RefMut}};

use super::{Component, components::{component_manager::ComponentManager, comp_pool::CompPool}, entities::Entity};


pub struct Query<'a> {
    component_manager: &'a ComponentManager<'a>,
}


impl<'a> Query<'a> {
    pub fn new(component_manager: &'a ComponentManager) -> Self {
        Self { component_manager }
    }

    pub fn get<T: Component + 'static>(self) -> Ref<'a, CompPool<T>>{
        self.component_manager.get_components::<T>().unwrap()
    }

    pub fn get_mut<T: Component +'static>(self) -> RefMut<'a, CompPool<T>> {
        self.component_manager.get_components_mut::<T>().unwrap()
    }
}

