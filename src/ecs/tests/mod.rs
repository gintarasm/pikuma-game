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

    struct Fps(pub u32);
}
