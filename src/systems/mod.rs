use crate::{ecs::{SystemAction, world::World, SystemBuilder, entities::Entity, System}, components::{TransformComponent, RigidBodyComponent}};


pub struct MovementSystem {}

impl SystemAction for MovementSystem {
    fn action(&self, world: &mut World, entities: &Vec<Entity>) {
        entities.iter().for_each(|entity| {
            let transform = world.get_component::<TransformComponent>(entity).unwrap();
            let rigid_body = world.get_component::<RigidBodyComponent>(entity).unwrap();

            // transform.position += rigid_body.velocity
        });
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("MovementSystem", self, world.get_component_signatures())
            .with_component::<TransformComponent>()
            .with_component::<RigidBodyComponent>()
            .build()
    }
}