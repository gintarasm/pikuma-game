use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, resource: impl Any) {
        let type_id = resource.type_id();
        self.data.insert(type_id, Box::new(resource));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        self.data.get(&type_id)?.downcast_ref()
    }

    pub fn get_ref_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        self.data.get_mut(&type_id)?.downcast_mut()
    }

    pub fn delete<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();

        self.data.remove(&type_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn add_resource() {
        let mut resources = Resources::new();
        let world_width = WorldWidth(100.0);
        let world_width_type_id = world_width.type_id();

        resources.add(world_width);

        let stored_resource = resources
            .data
            .get(&world_width_type_id)
            .unwrap()
            .downcast_ref::<WorldWidth>()
            .unwrap();

        assert_eq!(stored_resource.0, 100.0);
    }

    #[test]
    fn get_resource() {
        let mut resources = Resources::new();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);

        let stored_resource = resources.get_ref::<WorldWidth>().unwrap();

        assert_eq!(stored_resource.0, 100.0);
    }

    #[test]
    fn get_resource_mut() {
        let mut resources = Resources::new();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);
        {
            let stored_resource = resources.get_ref_mut::<WorldWidth>().unwrap();
            stored_resource.0 += 1.0;
        }

        let stored_resource = resources.get_ref::<WorldWidth>().unwrap();

        assert_eq!(stored_resource.0, 101.0);
    }

    #[test]
    fn delete_resource() {
        let mut resources = Resources::new();
        let world_width = WorldWidth(100.0);
        
        resources.add(world_width);
        resources.delete::<WorldWidth>();

        let stored_resource = resources.get_ref::<WorldWidth>();

        assert!(stored_resource.is_none());
    }

    struct WorldWidth(pub f32);
}
