use std::{any::Any, cell::RefCell};

use crate::ecs::{entities::Entity, errors::EcsErrors};

use super::Component;

pub trait GenericCompPool {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_empty(&self) -> bool;
    fn get_size(&self) -> usize;
    fn resize(&mut self, size: usize);
    fn clear(&mut self);
    fn remove_any(&mut self, entity: &Entity);
}

pub struct CompPool<T: Component> {
    pub data: Vec<Option<T>>,
}

impl<T: 'static + Component> GenericCompPool for RefCell<CompPool<T>> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_empty(&self) -> bool {
        self.borrow().data.is_empty()
    }

    fn get_size(&self) -> usize {
        self.borrow().data.len()
    }

    fn resize(&mut self, size: usize) {
        self.borrow_mut().data.resize_with(size, || None)
    }

    fn clear(&mut self) {
        self.borrow_mut().data.clear();
    }

    fn remove_any(&mut self, entity: &Entity) {
        self.borrow_mut().data[entity.0] = None;
    }
}

impl<T: Component + 'static> CompPool<T> {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, || None);

        Self { data }
    }

    pub fn add(&mut self, comp: T) {
        self.data.push(Some(comp));
    }

    pub fn remove(&mut self, index: usize) -> Result<(), EcsErrors> {
        if self.data.get(index).is_none() {
            return Err(EcsErrors::EntityDoesNotExist(index));
        }
        self.data[index] = None;
        Ok(())
    }

    pub fn set(&mut self, index: usize, comp: T) -> Result<(), EcsErrors> {
        if self.data.get(index).is_none() {
            return Err(EcsErrors::EntityDoesNotExist(index));
        }
        self.data[index] = Some(comp);

        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<&T, EcsErrors> {
        if let Some(comp) = self.data.get(index) {
            Ok(comp.as_ref().unwrap())
        } else {
            Err(EcsErrors::EntityDoesNotExist(index))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, EcsErrors> {
        if let Some(comp) = self.data.get_mut(index) {
            Ok(comp.as_mut().unwrap())
        } else {
            Err(EcsErrors::EntityDoesNotExist(index))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<T>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> {
        self.data.iter_mut()
    }
}
