use std::{
    any::{Any, TypeId},
    collections::HashMap, cell::RefCell,
};

pub struct Resource {
    data: Box<dyn Any>,
}

impl Resource {
    fn new<T: Any>(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    pub fn get<T: Any>(&self) -> &T {
        self.data.downcast_ref().unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.data.downcast_mut().unwrap()
    }
}

pub struct Resources {
    data: HashMap<TypeId, RefCell<Resource>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, resource: impl Any) {
        let type_id = resource.type_id();
        self.data.insert(type_id, RefCell::new(Resource::new(resource)));
    }

    pub fn get<T: Any>(&self) -> &RefCell<Resource> {
        let type_id = TypeId::of::<T>();

        self.data
            .get(&type_id)
            .unwrap()
    }

    pub fn delete<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();

        self.data.remove(&type_id);
    }
}
