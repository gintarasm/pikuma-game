#[cfg(test)]
use crate::ecs::world::World;

#[test]
fn create_entity() {
    let mut world = World::new();
    let entity = world.create_entity();
    assert_eq!(entity.0, 0);
}

#[cfg(test)]
mod resources {
    use ecs_macro::Component;

    use crate::ecs::world::World;

    #[test]
    fn create_and_get_resource_immutably() {
        let mut world = World::new();
        world.add_resource(Fps(60));
        if let Some(fps) = world.get_resource::<Fps>() {
            assert_eq!(fps.0, 60);
        }
    }

    #[test]
    fn get_mutable_resource() {
        let mut world = World::new();
        world.add_resource(Fps(60));
        {
            let fps = world.get_resource_mut::<Fps>().unwrap();
            fps.0 += 1;
        }
        let fps = world.get_resource::<Fps>().unwrap();
        assert_eq!(fps.0, 61);
    }

    #[test]
    fn delete_resource() {
        let mut world = World::new();
        world.add_resource(Fps(60));
        world.delete_resource::<Fps>();

        let resource = world.get_resource::<Fps>();

        assert!(resource.is_none());   
    }

    #[test]
    fn query_for_entities() {        
        let mut world = World::new();

        let entity = world.create_entity();
        world.add_component(&entity, Location(1, 1));
        world.add_component(&entity, Size(10));

        let entity2 = world.create_entity();
        world.add_component(&entity2, Location(11, 11));

        let entity3 = world.create_entity();
        world.add_component(&entity3, Size(99));

    }

    #[test]
    fn reuse_deleted_entity_ids() {
        let mut world = World::new();

        let entity = world.create_entity();
        world.update();
        
        world.remove_entity(&entity);
        world.update();

        let new_entity = world.create_entity();
        world.update();

        assert_eq!(entity.0, new_entity.0);
    }


    
    #[derive(Component)]
    struct Location(pub i32, pub i32);
    #[derive(Component)]
    struct Size(pub i32);

    struct Fps(pub u32);
}
