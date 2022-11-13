use crate::{ecs::{SystemAction, SystemBuilder}, components::{TransformComponent, RigidBodyComponent}};

pub struct MovementSystem {}

impl SystemAction for MovementSystem {
    fn action(&self, world: &mut crate::ecs::world::World, entities: &Vec<crate::ecs::Entity>) {
        entities.iter().for_each(|entity| {
            let transform = world.get_component::<TransformComponent>(entity).unwrap();
            let rigid_body = world.get_component::<RigidBodyComponent>(entity).unwrap();

            transform.position += rigid_body.velocity
        });
    }

    fn to_system(self) -> crate::ecs::System {
        SystemBuilder::new("MovementSystem", self)
        .with_component::<TransformComponent>()
        .with_component::<RigidBodyComponent>()
        .build()
    }
}